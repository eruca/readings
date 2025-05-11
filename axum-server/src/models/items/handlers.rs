use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::AppState;
use crate::errors::AppError;

use super::item::{CreateItemPayload, Item, UpdateItemPayload};

// --- 处理器函数 (Handlers) ---
#[instrument(skip(pool))] // 记录函数调用和参数 (除了 pool)
pub(crate) async fn create_item(
    State(pool): State<AppState>,
    Json(payload): Json<CreateItemPayload>,
) -> Result<impl IntoResponse, AppError> {
    debug!("Attempting to create item: {:?}", payload);
    if payload.name.is_empty() {
        return Err(AppError::InvalidInput(
            "Item name cannot be empty".to_string(),
        ));
    }

    let item = sqlx::query_as!(
        Item,
        // 使用参数化查询防止 SQL 注入
        r#"
        INSERT INTO items (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at, updated_at
        "#,
        payload.name,
        payload.description
    )
    .fetch_one(pool.as_ref()) // 使用 .as_ref() 从 Arc<PgPool> 获取 &PgPool
    .await?;

    debug!("Item created successfully: {:?}", item);
    Ok((StatusCode::CREATED, Json(item)))
}

#[instrument(skip(pool))]
pub(crate) async fn list_items(State(pool): State<AppState>) -> Result<Json<Vec<Item>>, AppError> {
    debug!("Fetching all items");
    let items = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool.as_ref())
    .await?;

    debug!("Found {} items", items.len());
    Ok(Json(items))
}

#[instrument(skip(pool))]
pub(crate) async fn get_item(
    State(pool): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<Json<Item>, AppError> {
    debug!("Fetching item with id: {}", item_id);
    let item = sqlx::query_as!(
        Item,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM items
        WHERE id = $1
        "#,
        item_id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    match item {
        Some(i) => {
            debug!("Item found: {:?}", i);
            Ok(Json(i))
        }
        None => {
            debug!("Item with id {} not found", item_id);
            Err(AppError::NotFound(item_id))
        }
    }
}

#[instrument(skip(pool, payload))]
pub(crate) async fn update_item(
    State(pool): State<AppState>,
    Path(item_id): Path<Uuid>,
    Json(payload): Json<UpdateItemPayload>,
) -> Result<Json<Item>, AppError> {
    debug!("Attempting to update item {}: {:?}", item_id, payload);
    // 首先检查项目是否存在
    let existing_item = sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", item_id)
        .fetch_optional(pool.as_ref())
        .await?;

    if existing_item.is_none() {
        return Err(AppError::NotFound(item_id));
    }

    // 如果字段是 Some，则更新，否则保持不变 (更复杂的逻辑可能需要动态构建查询)
    // 这个例子为了简单，假设总是更新，如果 payload 字段是 None，则数据库中对应字段可能被设为 NULL (如果允许)
    // 或者你需要更复杂的逻辑来只更新非 None 的字段。
    // 为了简单，这里我们要求 name 和 description 至少有一个被提供 (实际中你可能允许只更新一个)
    // 更健壮的方式是只更新 payload 中存在的字段
    let item = sqlx::query_as!(
        Item,
        r#"
        UPDATE items
        SET
            name = COALESCE($1, name),             -- 如果 payload.name 是 None，则保持旧值
            description = COALESCE($2, description) -- 如果 payload.description 是 None，则保持旧值
        WHERE id = $3
        RETURNING id, name, description, created_at, updated_at
        "#,
        payload.name,
        payload.description,
        item_id
    )
    .fetch_one(pool.as_ref())
    .await?;

    debug!("Item updated successfully: {:?}", item);
    Ok(Json(item))
}

#[instrument(skip(pool))]
pub(crate) async fn delete_item(
    State(pool): State<AppState>,
    Path(item_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    debug!("Attempting to delete item with id: {}", item_id);
    let result = sqlx::query!(
        r#"
        DELETE FROM items
        WHERE id = $1
        "#,
        item_id
    )
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        debug!("Item with id {} not found for deletion", item_id);
        Err(AppError::NotFound(item_id))
    } else {
        debug!("Item {} deleted successfully", item_id);
        Ok(StatusCode::NO_CONTENT)
    }
}
