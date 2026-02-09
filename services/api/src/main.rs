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
<html lang="ja" data-theme="pastel">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Futelt</title>
  <style>
    /* =========================
       Theme tokens
    ========================= */
    :root{
      --bg1:#fff1f6;
      --bg2:#eef7ff;

      --text:#2b2b2b;
      --muted:#6b7280;

      --surface: rgba(255,255,255,.86);
      --surface2: rgba(255,255,255,.72);
      --border: rgba(255,214,231,.90);

      --accent:#ff7aa2;
      --accent2:#7cc9ff;

      --shadow: 0 18px 45px rgba(255, 122, 162, .16);
      --shadow2: 0 12px 30px rgba(124,201,255,.10);
    }

    /* 🌸 pastel */
    html[data-theme="pastel"]{
      --bg1:#fff1f6;
      --bg2:#eef7ff;

      --text:#2b2b2b;
      --muted:#6b7280;

      --surface: rgba(255,255,255,.86);
      --surface2: rgba(255,255,255,.72);
      --border: rgba(255,214,231,.90);

      --accent:#ff7aa2;
      --accent2:#7cc9ff;

      --shadow: 0 18px 45px rgba(255, 122, 162, .16);
      --shadow2: 0 12px 30px rgba(124,201,255,.10);
    }

    /* 🍃 mint */
    html[data-theme="mint"]{
      --bg1:#eafff5;
      --bg2:#eef6ff;

      --text:#153028;
      --muted:#4b6b60;

      --surface: rgba(255,255,255,.86);
      --surface2: rgba(255,255,255,.72);
      --border: rgba(191,243,221,.95);

      --accent:#2dd4bf;
      --accent2:#60a5fa;

      --shadow: 0 18px 45px rgba(45, 212, 191, .14);
      --shadow2: 0 12px 30px rgba(96,165,250,.10);
    }

    /* 🌙 dark */
    html[data-theme="dark"]{
      --bg1:#0b0f19;
      --bg2:#0b0f19;

      --text:#eaf0ff;
      --muted:#9fb0d0;

      --surface: rgba(18,26,43,.78);
      --surface2: rgba(18,26,43,.58);
      --border: rgba(38,50,77,.95);

      --accent:#7c5cff;
      --accent2:#38bdf8;

      --shadow: 0 20px 55px rgba(0,0,0,.38);
      --shadow2: 0 14px 34px rgba(0,0,0,.28);
    }

    /* =========================
       Base
    ========================= */
    *{ box-sizing:border-box; }
    body{
      margin:0;
      font-family: "Noto Sans JP", ui-sans-serif, system-ui, -apple-system, "Segoe UI",
                   Roboto, "Hiragino Sans", "Yu Gothic", sans-serif;
      color:var(--text);
      background: transparent; /* 背景は fixed レイヤーで描く */
    }

    /* ✅ 背景を画面に固定（履歴が増えても見え方が変わらない） */
    body::before{
      content:"";
      position: fixed;
      inset: 0;
      z-index: -1;
      background:
        radial-gradient(1200px 600px at 20% 10%, var(--bg1), transparent 60%),
        radial-gradient(1200px 600px at 80% 20%, var(--bg2), transparent 60%),
        linear-gradient(180deg, rgba(0,0,0,.06), rgba(0,0,0,.10));
    }

    /* =========================
       Layout
    ========================= */
    .page{
      min-height:100vh;
      padding: 44px 18px;
      display:flex;
      justify-content:center;  /* ✅ 左寄り防止 */
      align-items:flex-start;
    }

    .card{
      width: min(1100px, 100%);
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: 28px;
      padding: 22px;
      box-shadow: var(--shadow);
      backdrop-filter: blur(10px);
    }

    .topbar{
      display:flex;
      gap:10px;
      flex-wrap:wrap;
      margin: 0 0 14px;
    }

    .chip{
      display:inline-flex;
      align-items:center;
      gap:8px;
      padding: 10px 14px;
      border-radius: 999px;
      border: 1px solid var(--border);
      background: var(--surface2);
      box-shadow: var(--shadow2);
      cursor:pointer;
      font-weight: 800;
      color: var(--text);
    }

    .header{
      display:flex;
      align-items:center;
      gap:14px;
      flex-wrap:wrap;
      margin: 6px 0 10px;
    }

    .logo{
      width: 54px;
      height: 54px;
      border-radius: 18px;
      background: linear-gradient(135deg, color-mix(in oklab, var(--accent) 80%, white 20%),
                                       color-mix(in oklab, var(--accent2) 78%, white 22%));
      display:grid;
      place-items:center;
      color:white;
      font-weight: 900;
      font-size: 26px;
      box-shadow: 0 14px 30px rgba(0,0,0,.15);
    }

    .title{
      margin:0;
      font-size: 44px;
      letter-spacing: .4px;
    }

    .badge{
      display:inline-block;
      font-size:12px;
      padding: 8px 12px;
      border-radius: 999px;
      border: 1px solid var(--border);
      background: color-mix(in oklab, var(--surface2) 70%, var(--accent) 30%);
      color: var(--text);
      font-weight: 800;
    }

    .sub{
      margin: 6px 0 18px;
      color: var(--muted);
      font-size: 15px;
      line-height: 1.7;
    }

    .row{
      display:flex;
      gap:12px;
      align-items:center;
      margin-top: 6px;
    }

    input{
      flex:1;
      padding: 14px 16px;
      border-radius: 18px;
      border: 1px solid var(--border);
      background: var(--surface2);
      color: var(--text);
      outline: none;
      box-shadow: var(--shadow2);
    }
    input::placeholder{ color: color-mix(in oklab, var(--muted) 85%, transparent); }
    input:focus{
      border-color: color-mix(in oklab, var(--accent) 70%, var(--border));
      box-shadow: 0 0 0 4px color-mix(in oklab, var(--accent) 22%, transparent);
    }

    button.primary{
      padding: 14px 18px;
      border-radius: 18px;
      border: 0;
      background: linear-gradient(90deg, var(--accent), color-mix(in oklab, var(--accent) 65%, white 35%));
      color: white;
      font-weight: 900;
      cursor:pointer;
      box-shadow: 0 14px 30px color-mix(in oklab, var(--accent) 24%, transparent);
    }
    button.primary:active{ transform: translateY(1px); }

    button.secondary{
      padding: 12px 14px;
      border-radius: 18px;
      border: 1px solid var(--border);
      background: var(--surface2);
      color: var(--text);
      font-weight: 800;
      cursor:pointer;
      box-shadow: var(--shadow2);
    }

    .meta{
      margin-top: 14px;
      display:flex;
      align-items:center;
      gap: 12px;
    }
    #status{
      color: var(--muted);
      font-size: 13px;
    }

    .hint{
      margin-top: 10px;
      color: var(--muted);
      font-size: 12px;
    }

    h2{
      margin: 20px 0 12px;
      font-size: 20px;
      letter-spacing: .2px;
    }

    ul{
      list-style:none;
      padding:0;
      margin:0;
      display:flex;
      flex-direction:column;
      gap:12px;
    }

    li{
      padding: 14px 14px;
      border-radius: 18px;
      background: var(--surface2);
      border: 1px solid var(--border);
      box-shadow: var(--shadow2);
      color: var(--text);
      word-break: break-word;
    }

    .history-item {
    transition: transform 0.15s ease, box-shadow 0.15s ease;
    }
    .history-item:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(0,0,0,0.08);
    }

    /* little responsive */
    @media (max-width: 720px){
      .title{ font-size: 34px; }
      .logo{ width:46px; height:46px; font-size:22px; border-radius:16px; }
      .row{ flex-direction: column; align-items: stretch; }
      button.primary{ width: 100%; }
    }
  </style>
</head>

<body>
  <div class="page">
    <div class="card">
      <div class="topbar">
        <button type="button" class="chip" data-theme-btn="pastel">🌸 パステル</button>
        <button type="button" class="chip" data-theme-btn="mint">🍃 ミント</button>
        <button type="button" class="chip" data-theme-btn="dark">🌙 ダーク</button>
      </div>

      <div class="header">
        <div class="logo">F</div>
        <h1 class="title">Futelt</h1>
        <span class="badge">未来の自分チャット ✨</span>
      </div>

      <p class="sub">
        今のあなたから、未来のあなたへ。<br>
        ちいさなメッセージを残して、あとで読み返そう😊
      </p>

      <form id="form" class="row">
        <input id="text" placeholder="未来の自分へメッセージ…" />
        <button class="primary" type="submit">送信</button>
      </form>

      <div class="meta">
        <button id="reload" class="secondary" type="button">更新</button>
        <span id="status"></span>
      </div>

      <div class="hint">※いまは試作版：サーバー再起動で履歴は消えます</div>

      <h2>履歴</h2>
      <ul id="list"></ul>
    </div>
  </div>

<script>
(function initTheme(){
  const saved = localStorage.getItem('theme') || 'pastel';
  document.documentElement.setAttribute('data-theme', saved);

  document.querySelectorAll('[data-theme-btn]').forEach(btn => {
    btn.addEventListener('click', () => {
      const t = btn.getAttribute('data-theme-btn');
      document.documentElement.setAttribute('data-theme', t);
      localStorage.setItem('theme', t);
    });
  });
})();

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
