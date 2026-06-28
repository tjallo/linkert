#[derive(Debug, sqlx::FromRow)]
pub struct RefreshToken {
    pub id: i64,
    pub user_id: i64,
    pub refresh_token: String,
    pub user_agent: Option<String>,
    pub device_name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}
