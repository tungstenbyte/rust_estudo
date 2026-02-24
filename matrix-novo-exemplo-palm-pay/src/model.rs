use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsPage<T> {
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
    pub items: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Meuexemplo {
    pub id: i64,
    pub status_code: String,
    pub name: String,
    pub description: String,
    pub allows_transactions: bool,
    pub max_transaction_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MeuexemploRow {
    pub id: i64,
    pub status_code: String,
    pub name: String,
    pub description: String,
    pub allows_transactions: bool,
    pub max_transaction_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
