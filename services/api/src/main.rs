use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
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

async fn index() -> Html<&'static str> {
    Html(r#"
<!doctype html>
<html lang="ja">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Futelt</title>
</head>
<body style="font-family: system-ui; max-width: 720px; margin: 24px auto; padding: 0 12px;">
  <h1>Futelt（未来の自分チャット）</h1>

  <form id="form" style="display:flex; gap:8px; margin: 16px 0;">
    <input id="text" placeholder="未来の自分へメッセージ…" style="flex:1; padding:10px;" />
    <button type="submit" style="padding:10px 14px;">送信</button>
  </form>

  <div style="display:flex; gap:8px; align-items:center;">
    <button id="reload" style="padding:8px 12px;">更新</button>
    <span id="status" style="color:#666;"></span>
  </div>

  <h2 style="margin-top: 18px;">履歴</h2>
  <ul id="list"></ul>

<script>
async function load() {
  const status = document.getElementById('status');
  status.textContent = '読み込み中…';
  const res = await fetch('/messages');
  const data = await res.json();
  const list = document.getElementById('list');
  list.innerHTML = '';
  for (const item of data.items) {
    const li = document.createElement('li');
    li.textContent = `#${item.id} ${item.text}`;
    list.appendChild(li);
  }
  status.textContent = `OK（${data.items.length}件）`;
}

document.getElementById('reload').addEventListener('click', load);

document.getElementById('form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const input = document.getElementById('text');
  const text = input.value.trim();
  if (!text) return;

  const res = await fetch('/messages', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({text})
  });

  if (!res.ok) {
    alert('送信に失敗しました');
    return;
  }

  input.value = '';
  await load();
});

load();
</script>
</body>
</html>
"#)
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
        .route("/", get(index))
        .route("/health", get(health))
        .route("/messages", post(create_message).get(list_messages))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3001";
    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
