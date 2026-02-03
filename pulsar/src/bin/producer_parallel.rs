//! Produtor com concorrência e paralelismo: N tasks (workers) que enviam
//! mensagens em paralelo, cada uma com seu próprio producer. As mensagens
//! são distribuídas entre as tasks (ex.: task 0 envia 1, 5, 9, ...; task 1
//! envia 2, 6, 10, ...).

use anyhow::Result;
use pulsar::{
    message::proto, producer, Pulsar, TokioExecutor,
};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

/// Número de producers/tasks em paralelo.
const NUM_WORKERS: usize = 10;

/// Total de mensagens a enviar (distribuídas entre as tasks).
const TOTAL_MESSAGES: u64 = 100_000;

#[derive(Serialize)]
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
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,pulsar=off")),
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

    info!("Criando {} producers para o tópico: {}", NUM_WORKERS, topic);

    let sent_count = Arc::new(AtomicU64::new(0));
    let start = Instant::now();

    let mut handles = Vec::with_capacity(NUM_WORKERS);

    for worker_id in 0..NUM_WORKERS {
        let topic_clone = topic.clone();
        let pulsar = pulsar.clone();
        let sent = sent_count.clone();

        let handle = tokio::spawn(async move {
            let mut producer = pulsar
                .producer()
                .with_topic(topic_clone)
                .with_name(format!("rust-producer-parallel-{}", worker_id))
                .with_options(producer::ProducerOptions {
                    schema: Some(proto::Schema {
                        r#type: proto::schema::Type::String as i32,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build()
                .await
                .expect("build producer");

            let mut n = 0u64;
            let worker_id_u = worker_id as u64;
            // Task worker_id envia ids: worker_id+1, worker_id+1+N, worker_id+1+2*N, ...
            let mut k = 0u64;
            loop {
                let id = worker_id_u + 1 + k * (NUM_WORKERS as u64);
                if id > TOTAL_MESSAGES {
                    break;
                }
                n += 1;
                let message_data = Message {
                    id,
                    content: format!("Mensagem número {} (worker {})", id, worker_id),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };
                let json = serde_json::to_string(&message_data).expect("serialize");
                producer
                    .send(producer::Message {
                        payload: json.into_bytes(),
                        ..Default::default()
                    })
                    .await
                    .expect("send");
                let c = sent.fetch_add(1, Ordering::SeqCst) + 1;
                if c % 10_000 == 0 {
                    info!("Producer: {} mensagens enviadas", c);
                }
                k += 1;
            }
            n
        });

        handles.push(handle);
    }

    for h in handles {
        let _ = h.await.expect("join");
    }

    let elapsed = start.elapsed();
    info!(
        "Producer: {} mensagens enviadas. Tempo total de execução: {:?}",
        sent_count.load(Ordering::SeqCst),
        elapsed
    );

    Ok(())
}
