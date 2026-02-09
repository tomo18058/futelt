use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use domain::Message;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    messages: Arc<Mutex<Vec<Message>>>,
}

#[derive(Deserialize)]
struct CreateMessageRequest {
    text: String,
}

#[derive(Serialize)]
struct CreateMessageResponse {
    id: u64,
}

#[derive(Serialize)]
struct ListMessagesResponse {
    items: Vec<MessageItem>,
}

#[derive(Serialize)]
struct MessageItem {
    id: u64,
    text: String,
}

async fn health() -> &'static str {
    "ok"
}

async fn create_message(
    State(state): State<AppState>,
    Json(req): Json<CreateMessageRequest>,
) -> (StatusCode, Json<CreateMessageResponse>) {
    let mut messages = state.messages.lock().unwrap();
    let id = (messages.len() as u64) + 1;

    messages.push(Message {
        id,
        text: req.text,
    });

    (StatusCode::CREATED, Json(CreateMessageResponse { id }))
}

async fn list_messages(State(state): State<AppState>) -> Json<ListMessagesResponse> {
    let messages = state.messages.lock().unwrap();
    let items = messages
        .iter()
        .cloned()
        .map(|m| MessageItem { id: m.id, text: m.text })
        .collect();

    Json(ListMessagesResponse { items })
}

#[tokio::main]
async fn main() {
    let state = AppState {
        messages: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/messages", post(create_message).get(list_messages))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3001";
    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
