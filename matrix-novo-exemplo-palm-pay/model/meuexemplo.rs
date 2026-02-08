// Equivalente ao model.Meuexemplo
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