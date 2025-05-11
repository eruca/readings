use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// --- 数据模型 ---
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub(crate) struct Item {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateItemPayload {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateItemPayload {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}
