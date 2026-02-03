//! Consumidor com **pool fixo de workers** (estilo Go): N workers fixos que
//! ficam em loop recebendo mensagens de um canal; um receptor recebe do Pulsar
//! e distribui em round-robin para os N canais (um por worker). Cada worker
//! processa e envia a mensagem de volta para o receptor fazer o ack.
//!
//! Equivalente em Go:
//!   jobs := make(chan Message, cap)
//!   for i := 0; i < N; i++ { go func() { for msg := range jobs { process(msg) } }() }
//!   for msg := range pulsar.Messages() { jobs <- msg }

use anyhow::Result;
use futures_util::TryStreamExt;
use pulsar::{
    message::proto, consumer, Pulsar, SubType, TokioExecutor,
};
use pulsar::consumer::InitialPosition;
use serde::Deserialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::{info, error};

const NUM_WORKERS: usize = 10;
/// Capacidade dos canais: maior = menos bloqueio; nunca bloquear no send para não deixar de processar acks.
const CHANNEL_CAP: usize = 256;

type PulsarMsg = pulsar::consumer::Message<Vec<u8>>;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Message {
    id: u64,
    content: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let pulsar_url = std::env::var("PULSAR_URL")
        .unwrap_or_else(|_| "pulsar://localhost:6650".to_string());
    let topic = std::env::var("PULSAR_TOPIC")
        .unwrap_or_else(|_| "persistent://public/default/estudo_topic".to_string());

    info!("Conectando ao Pulsar em: {}", pulsar_url);

    let pulsar: Pulsar<_> = Pulsar::builder(pulsar_url, TokioExecutor)
        .build()
        .await?;

    info!("Criando consumidor para o tópico: {}", topic);

    let mut consumer = pulsar
        .consumer()
        .with_topic(topic)
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("rust-subscription-workers")
        .with_consumer_name("rust-consumer-workers")
        .with_batch_size(10_000) // mais permits do broker = mais mensagens em voo; evita travamento por flow control
        .with_options(consumer::ConsumerOptions {
            schema: Some(proto::Schema {
                r#type: proto::schema::Type::String as i32,
                ..Default::default()
            }),
            initial_position: InitialPosition::Earliest, // consome desde o início (inclui mensagens já publicadas)
            ..Default::default()
        })
        .build::<Vec<u8>>()
        .await?;

    info!(
        "Consumidor criado. Pool fixo de {} workers (estilo Go).",
        NUM_WORKERS
    );

    let start = Instant::now();

    // Canal receptor -> dispatcher (mensagens do Pulsar)
    let (pulsar_tx, mut pulsar_rx) = mpsc::channel::<PulsarMsg>(CHANNEL_CAP);
    // N canais dispatcher -> worker (um por worker)
    let mut worker_txs: Vec<mpsc::Sender<PulsarMsg>> = Vec::with_capacity(NUM_WORKERS);
    let mut worker_rxs = Vec::with_capacity(NUM_WORKERS);
    for _ in 0..NUM_WORKERS {
        let (tx, rx) = mpsc::channel::<PulsarMsg>(CHANNEL_CAP);
        worker_txs.push(tx);
        worker_rxs.push(rx);
    }
    // Canal workers -> receptor (para ack): capacidade alta para workers nunca bloquearem
    const ACK_CHANNEL_CAP: usize = 50_000;
    let (ack_tx, mut ack_rx) = mpsc::channel::<PulsarMsg>(ACK_CHANNEL_CAP);

    let message_count = Arc::new(AtomicUsize::new(0));
    let message_count_final = message_count.clone();

    // Dispatcher: recebe do receptor e distribui em round-robin para os N workers
    let dispatch_handle = tokio::spawn(async move {
        let mut idx = 0usize;
        while let Some(msg) = pulsar_rx.recv().await {
            let i = idx % NUM_WORKERS;
            idx = idx.wrapping_add(1);
            if let Err(e) = worker_txs[i].send(msg).await {
                error!("worker channel {} send: {}", i, e);
                eprintln!("ERRO consumer_workers [dispatcher]: worker channel {} send: {}", i, e);
                break;
            }
        }
    });

    // N workers fixos (estilo Go: for msg := range jobs { process(msg) })
    let mut worker_handles = Vec::with_capacity(NUM_WORKERS);
    for (worker_id, mut worker_rx) in worker_rxs.into_iter().enumerate() {
        let tx = ack_tx.clone();
        let count = message_count.clone();
        let h = tokio::spawn(async move {
            while let Some(msg) = worker_rx.recv().await {
                let n = count.fetch_add(1, Ordering::SeqCst);
                match std::str::from_utf8(&msg.payload.data) {
                    Ok(json_str) => {
                        match serde_json::from_str::<Message>(json_str) {
                            Ok(_m) => {
                                // Sem print por mensagem; progresso a cada 10k no receptor
                            }
                            Err(e) => {
                                info!("worker {} mensagem #{} (raw): {}", worker_id, n + 1, json_str);
                                error!("worker {} deserialize: {}", worker_id, e);
                            }
                        }
                    }
                    Err(e) => error!("worker {} utf8: {}", worker_id, e),
                }
                if tx.send(msg).await.is_err() {
                    break;
                }
            }
        });
        worker_handles.push(h);
    }
    drop(ack_tx);

    // Receptor: recebe do Pulsar e manda para o dispatcher; recebe acks e confirma.
    // Nunca bloquear em pulsar_tx.send(): se o canal estiver cheio, guardamos a mensagem
    // e continuamos processando acks (senão o broker para de enviar por flow control).
    let mut received_count: u64 = 0;
    let mut pending_msg: Option<PulsarMsg> = None;
    let mut wait_interval = interval(Duration::from_secs(10));
    wait_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    info!("Aguardando mensagens do tópico (Earliest = desde o início)...");
    // Print só a cada 10k; sem "Ainda aguardando" a cada 10s para não poluir
    loop {
        // Drenar todos os acks disponíveis antes de receber mais: evita broker parar por flow control
        while let Ok(msg) = ack_rx.try_recv() {
            if let Err(e) = consumer.ack(&msg).await {
                let err_msg = format!("ack: {:#}", e);
                error!("{}", err_msg);
                eprintln!("ERRO consumer_workers [ack]: {}", err_msg);
            }
        }

        // Esvaziar buffer antes de receber mais: try_send para não bloquear
        if let Some(m) = pending_msg.take() {
            match pulsar_tx.try_send(m) {
                Ok(()) => {}
                Err(TrySendError::Full(m)) => {
                    pending_msg = Some(m);
                }
                Err(TrySendError::Closed(_)) => {
                    info!("Canal para dispatcher fechado; encerrando loop de recepção.");
                    break;
                }
            }
        }

        tokio::select! {
            // Sempre ter um recv() para não fechar o branch; acks já drenados acima
            Some(msg) = ack_rx.recv() => {
                if let Err(e) = consumer.ack(&msg).await {
                    let err_msg = format!("ack: {:#}", e);
                    error!("{}", err_msg);
                    eprintln!("ERRO consumer_workers [ack]: {}", err_msg);
                }
            }
            _ = wait_interval.tick() => {
                // Heartbeat a cada 10s (sem log para não poluir; só 10k e tempo final)
            }
            msg = consumer.try_next(), if pending_msg.is_none() => {
                let msg = match msg {
                    Ok(Some(m)) => m,
                    Ok(None) => continue,
                    Err(e) => {
                        let err_msg = format!("try_next: {:#}", e);
                        error!("{}", err_msg);
                        eprintln!("ERRO consumer_workers [try_next]: {}", err_msg);
                        break;
                    }
                };
                received_count += 1;
                if received_count % 10_000 == 0 {
                    info!(
                        "Consumer: {} mensagens recebidas (tempo decorrido: {:?})",
                        received_count,
                        start.elapsed()
                    );
                }
                match pulsar_tx.try_send(msg) {
                    Ok(()) => {}
                    Err(TrySendError::Full(m)) => {
                        pending_msg = Some(m);
                    }
                    Err(TrySendError::Closed(_)) => {
                        info!("Canal para dispatcher fechado; encerrando loop de recepção.");
                        break;
                    }
                }
            }
            else => {
                info!("Canal de acks fechado (workers/dispatcher encerraram); encerrando loop.");
                break;
            }
        }
    }

    let elapsed = start.elapsed();
    info!(
        "Consumer: {} mensagens processadas. Tempo total de execução: {:?}",
        message_count_final.load(Ordering::SeqCst),
        elapsed
    );

    // Aguardar tasks para não perder panics/erros
    if let Err(e) = dispatch_handle.await {
        let err_msg = format!("dispatcher task panic/erro: {:?}", e);
        error!("{}", err_msg);
        eprintln!("ERRO consumer_workers [dispatcher join]: {}", err_msg);
    }
    for (i, h) in worker_handles.into_iter().enumerate() {
        if let Err(e) = h.await {
            let err_msg = format!("worker {} task panic/erro: {:?}", i, e);
            error!("{}", err_msg);
            eprintln!("ERRO consumer_workers [worker {} join]: {}", i, err_msg);
        }
    }

    Ok(())
}
