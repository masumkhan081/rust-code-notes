use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    next_id: Arc<AtomicU64>,
    items: Arc<RwLock<HashMap<u64, Item>>>,
}

#[derive(Debug, Serialize, Clone)]
struct Item {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateItem {
    name: String,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("item not found")]
    NotFound,
    #[error("invalid input: {0}")]
    BadInput(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadInput(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(ErrorBody { error: msg })).into_response()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_max_level(Level::INFO)
        .init();

    let state = AppState {
        next_id: Arc::new(AtomicU64::new(1)),
        items: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/items", post(create_item))
        .route("/items/:id", get(get_item))
        .with_state(state);

    let addr = "127.0.0.1:3000";
    info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn healthz() -> &'static str {
    "ok"
}

async fn create_item(
    State(state): State<AppState>,
    Json(req): Json<CreateItem>,
) -> Result<Json<Item>, ApiError> {
    if req.name.trim().is_empty() {
        return Err(ApiError::BadInput("name must not be empty".to_string()));
    }

    let id = state.next_id.fetch_add(1, Ordering::Relaxed);
    let item = Item { id, name: req.name };

    state.items.write().await.insert(id, item.clone());
    Ok(Json(item))
}

async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Item>, ApiError> {
    let guard = state.items.read().await;
    let item = guard.get(&id).cloned().ok_or(ApiError::NotFound)?;
    Ok(Json(item))
}
