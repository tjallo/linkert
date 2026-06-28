use serde::Serialize;
use utoipa::{
    ToSchema,
    openapi::{
        RefOr, Schema,
        schema::{ObjectBuilder, SchemaType, Type},
    },
};

pub mod auth;
pub mod health;

fn literal_true() -> RefOr<Schema> {
    ObjectBuilder::new()
        .schema_type(SchemaType::new(Type::Boolean))
        .enum_values(Some(vec![serde_json::Value::Bool(true)]))
        .build()
        .into()
}

fn literal_false() -> RefOr<Schema> {
    ObjectBuilder::new()
        .schema_type(SchemaType::new(Type::Boolean))
        .enum_values(Some(vec![serde_json::Value::Bool(false)]))
        .build()
        .into()
}

#[derive(Serialize, ToSchema)]
pub struct ErrorEnvelope {
    #[schema(schema_with = literal_false)]
    pub success: bool,
    pub error: String,
}

impl ErrorEnvelope {
    pub fn new(error: String) -> Self {
        Self {
            success: false,
            error,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct SuccessEnvelope<T: ToSchema> {
    #[schema(schema_with = literal_true)]
    pub success: bool,
    pub data: T,
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
