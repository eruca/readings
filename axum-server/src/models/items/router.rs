use axum::{Router, routing::get};

use super::handlers::{create_item, delete_item, get_item, list_items, update_item};
use crate::AppState;

pub(crate) fn create_router() -> Router<AppState> {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/{id}",
            get(get_item).put(update_item).delete(delete_item),
        )
}
