use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Segundominio {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SegundominioRow {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
