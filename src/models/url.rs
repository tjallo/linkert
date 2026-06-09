#[derive(Debug, sqlx::FromRow)]
pub struct Url {
    pub id: i64,
    pub stub: String,
    pub original_url: String,
    pub user_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub invalid_by: Option<chrono::DateTime<chrono::Utc>>,
}
