// lib.rs - Arquivo principal do módulo

pub mod model;
pub mod repository;
pub mod service;

// Re-exports para facilitar o uso
pub use model::*;
pub use repository::*;
pub use service::*;

// src/model.rs
// Este arquivo contém as estruturas de dados equivalentes ao package model do Go

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsPage<T> {
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
    pub items: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Meuexemplo {
    pub id: i64,
    pub status_code: String,
    pub name: String,
    pub description: String,
    pub allows_transactions: bool,
    pub max_transaction_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    #[sqlx(rename = "meuexemplo_category_full_count")]
    pub full_count: i64,
}

// src/repository.rs
// Este arquivo contém o código do repository que já convertemos

pub use crate::meuexemplo_repository_rust::*;

// src/service.rs  
// Este arquivo contém o código do service que acabamos de converter

pub use crate::meuexemplo_service_rust::*;

// main.rs ou onde você vai usar
// Exemplo de como usar os módulos

use std::sync::Arc;
use sqlx::PgPool;

async fn setup_application() -> Result<(), anyhow::Error> {
    // Setup do banco de dados
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost/db".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    let pg_pool = Arc::new(pool);

    // Setup do logger
    let logger = Box::new(DefaultLogger);

    // Criação do repository
    let repository = Arc::new(MeuexemploRepository::new(
        pg_pool.clone(),
        pg_pool.clone(),
        logger.clone(),
    ));

    // Criação do service
    let service = Resource::new(repository, logger);

    // Exemplo de uso
    let items_page = service.get_meuexemplo(0, 10).await?;
    println!("Found {} items", items_page.items.len());

    let meuexemplo = service.get_meuexemplo_by_id(1).await?;
    if let Some(item) = meuexemplo {
        println!("Found item: {}", item.name);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Configurar tracing/logging
    tracing_subscriber::fmt::init();

    // Setup da aplicação
    setup_application().await?;

    Ok(())
}