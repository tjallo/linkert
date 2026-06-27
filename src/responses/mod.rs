use serde::Serialize;
use utoipa::ToSchema;

pub mod auth;
pub mod health;

#[derive(Serialize, ToSchema)]
pub struct ErrorEnvelope {
    pub success: bool,
    pub error: String,
}

impl ErrorEnvelope {
    pub fn new(error: String) -> Self {
        Self { success: false, error }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ResponseEnvelope<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ResponseEnvelope<T> {
    pub fn ok(data: T) -> ResponseEnvelope<T> {
        return ResponseEnvelope {
            data: Some(data),
            success: true,
            error: None,
        };
    }

    pub fn err(error: String) -> ResponseEnvelope<T> {
        return ResponseEnvelope {
            data: None,
            success: false,
            error: Some(error),
        };
    }
}
