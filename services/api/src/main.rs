use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::Row;
use std::{path::PathBuf, str::FromStr};


#[derive(Clone)]
struct AppState {
    db: SqlitePool,
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

    html[data-theme="pastel"]{
      --bg1:#fff1f6; --bg2:#eef7ff;
      --text:#2b2b2b; --muted:#6b7280;
      --surface: rgba(255,255,255,.86);
      --surface2: rgba(255,255,255,.72);
      --border: rgba(255,214,231,.90);
      --accent:#ff7aa2; --accent2:#7cc9ff;
      --shadow: 0 18px 45px rgba(255, 122, 162, .16);
      --shadow2: 0 12px 30px rgba(124,201,255,.10);
    }

    html[data-theme="mint"]{
      --bg1:#eafff5; --bg2:#eef6ff;
      --text:#153028; --muted:#4b6b60;
      --surface: rgba(255,255,255,.86);
      --surface2: rgba(255,255,255,.72);
      --border: rgba(191,243,221,.95);
      --accent:#2dd4bf; --accent2:#60a5fa;
      --shadow: 0 18px 45px rgba(45, 212, 191, .14);
      --shadow2: 0 12px 30px rgba(96,165,250,.10);
    }

    html[data-theme="dark"]{
      --bg1:#0b0f19; --bg2:#0b0f19;
      --text:#eaf0ff; --muted:#9fb0d0;
      --surface: rgba(18,26,43,.78);
      --surface2: rgba(18,26,43,.58);
      --border: rgba(38,50,77,.95);
      --accent:#7c5cff; --accent2:#38bdf8;
      --shadow: 0 20px 55px rgba(0,0,0,.38);
      --shadow2: 0 14px 34px rgba(0,0,0,.28);
    }

    *{ box-sizing:border-box; }
    body{
      margin:0;
      font-family: "Noto Sans JP", ui-sans-serif, system-ui, -apple-system, "Segoe UI",
                   Roboto, "Hiragino Sans", "Yu Gothic", sans-serif;
      color:var(--text);
      background: transparent;
    }

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

    .page{
      min-height:100vh;
      padding: 44px 18px;
      display:flex;
      justify-content:center;
      align-items:flex-start;
    }

    /* ✅ ここが「きゅっ」を直す本体 */
    .wrap{
      width: 100%;
      max-width: 1200px;
      margin: 0 auto;
      padding: 48px 40px;
    }

    .card{
      width: 100%;
      background: transparent;
      border: 0;
      border-radius: 0;
      padding: 0;
      box-shadow: none;
      backdrop-filter: none;
      min-height: calc(100vh - 64px);
    }

    .topbar{
      display:flex;
      gap:10px;
      flex-wrap:wrap;
      margin: 0 0 14px;
      justify-content:center;
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
      justify-content:center;
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
      text-align:center;
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
      text-align:center;
    }

    /* ✅ 入力欄を広く：row を flex にする */
    .row{
      width: 100%;
      max-width: 900px;
      margin: 0 auto;
      display:flex;
      gap: 12px;
      align-items:center;
    }

    input, textarea{
      flex:1;
      min-width: 0;
      padding: 14px 16px;
      border-radius: 18px;
      border: 1px solid var(--border);
      background: var(--surface2);
      color: var(--text);
      outline: none;
      box-shadow: var(--shadow2);
      font: inherit;
    }

    textarea{
      resize: none;
      overflow: hidden;
      line-height: 1.6;
    }

    input::placeholder, textarea::placeholder{
      color: color-mix(in oklab, var(--muted) 85%, transparent);
    }

    input:focus, textarea:focus{
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
      white-space: nowrap;
    }
      
    button.primary:active{ 
      transform: translateY(1px); 
    }

    button.primary:disabled{
      opacity: .6;
      cursor: not-allowed;
      transform: none;
      box-shadow: none;
    }

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
      justify-content:center;
    }
    #status{
      color: var(--muted);
      font-size: 13px;
    }

    .hint{
      margin-top: 10px;
      color: var(--muted);
      font-size: 12px;
      text-align:center;
    }

    h2{
      margin: 20px 0 12px;
      font-size: 20px;
      letter-spacing: .2px;
      text-align:center;
    }

    /* ✅ 履歴も同じ幅で揃える */
    ul{
      list-style:none;
      padding:0;
      margin:0 auto;
      display:flex;
      flex-direction:column;
      gap:12px;
      width: 100%;
      max-width: 900px;
    }

    li{
      padding: 14px 14px;
      border-radius: 18px;
      background: var(--surface2);
      border: 1px solid var(--border);
      box-shadow: var(--shadow2);
      color: var(--text);
      word-break: break-word;
      position: relative;
      animation: fadeUp .25s ease;
      white-space: pre-wrap;
    }

    @keyframes fadeUp {
    from { opacity:0; transform: translateY(6px); }
    to   { opacity:1; transform:none; }
    }

    li::before {
    position: absolute;
    left: -28px;
    }

    @media (max-width: 720px){
      .wrap{ padding: 34px 16px; }
      .title{ font-size: 34px; }
      .logo{ width:46px; height:46px; font-size:22px; border-radius:16px; }
      .row{ flex-direction: column; align-items: stretch; }
      button.primary{ width: 100%; }
    }
  </style>
</head>

<body>
  <div class="page">
    <div class="wrap">
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
          <textarea id="text" rows="1" placeholder="未来の自分へメッセージ…"></textarea>
          <button id="submit" class="primary" type="submit">送信</button>
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

const input = document.getElementById('text');

function autosize(el){
  el.style.height = 'auto';
  el.style.height = el.scrollHeight + 'px';
}

input.addEventListener('input', () => autosize(input));

input.addEventListener('keydown', async (e) => {
  // Enter単体 = 送信、Shift+Enter = 改行
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault();
    document.getElementById('form').requestSubmit();
  }
});

// 初期状態でも1行に整える
autosize(input);


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

let isSending = false;

document.getElementById('form').addEventListener('submit', async (e) => {
  e.preventDefault();

  if (isSending) return; // 二重送信防止

  const input = document.getElementById('text');
  const button = document.getElementById('submit');
  const status = document.getElementById('status');

  const text = input.value.trim();
  if (!text) return;

  try {
    isSending = true;
    button.disabled = true;
    button.textContent = '送信中…';
    status.textContent = '送信中…';

    const res = await fetch('/messages', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({ text })
    });

    if (!res.ok) {
      alert('送信に失敗しました');
      return;
    }

    input.value = '';
    await load();
  } finally {
    isSending = false;
    button.disabled = false;
    button.textContent = '送信';
  }
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
    // INSERT
    let result = sqlx::query("INSERT INTO messages (text) VALUES (?1)")
        .bind(req.text)
        .execute(&state.db)
        .await
        .unwrap();

    let id = result.last_insert_rowid() as u64;
    (StatusCode::CREATED, Json(CreateMessageResponse { id }))
}

async fn list_messages(State(state): State<AppState>) -> Json<ListMessagesResponse> {
    let rows = sqlx::query("SELECT id, text FROM messages ORDER BY id DESC")
        .fetch_all(&state.db)
        .await
        .unwrap();

    let items = rows
        .into_iter()
        .map(|row| MessageItem {
            id: row.get::<i64, _>("id") as u64,
            text: row.get::<String, _>("text"),
        })
        .collect();

    Json(ListMessagesResponse { items })
}

#[tokio::main]
async fn main() {
    // ✅ api クレートの場所（= services/api）を基準にして、必ず workspace の data/ に落とす
    let db_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/futelt.db");

    // 親ディレクトリ（data/）を作る
    std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();

    // ✅ 「URL文字列」じゃなくて「ファイルパス」を options で渡す（ここが最強）
    let connect_opts = SqliteConnectOptions::from_str(db_path.to_str().unwrap())
        .unwrap()
        .create_if_missing(true);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .unwrap();

    // テーブル作成（無ければ）
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        );
        "#,
    )
    .execute(&db)
    .await
    .unwrap();

    let state = AppState { db };

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
