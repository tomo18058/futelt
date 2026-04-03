use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::{fs, path::PathBuf, str::FromStr};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Deserialize)]
struct CreateMessageRequest {
    text: String,
    future: String,
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
    reply: String,
    future: String,
}

#[derive(Deserialize)]
struct CreateDailyEntryRequest {
    action: String,
    mood: String,
    goal: String,
    reflection: String,
}

#[derive(Serialize)]
struct DailyEntry {
    id: u64,
    action: String,
    mood: String,
    goal: String,
    reflection: String,
    created_at: String,
}

async fn health() -> &'static str {
    "ok"
}

async fn index() -> Html<String> {
    let html_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("templates/index.html");

    let html = fs::read_to_string(&html_path)
        .expect("HTMLファイルが読めない");

    Html(html)
}

async fn create_message(
    State(state): State<AppState>,
    Json(req): Json<CreateMessageRequest>,
) -> (StatusCode, Json<CreateMessageResponse>) {
    let history_rows = sqlx::query("SELECT text FROM messages ORDER BY id DESC LIMIT 5")
        .fetch_all(&state.db)
        .await
        .unwrap();

    let history: Vec<String> = history_rows
        .into_iter()
        .map(|row| row.get::<String, _>("text"))
        .collect();

    let reply = generate_reply(&req.text, &history, &req.future);

    let result = sqlx::query("INSERT INTO messages (text, reply, future) VALUES (?1, ?2, ?3)")
        .bind(req.text)
        .bind(reply)
        .bind(req.future)
        .execute(&state.db)
        .await
        .unwrap();

    let id = result.last_insert_rowid() as u64;
    (StatusCode::CREATED, Json(CreateMessageResponse { id }))
}

async fn list_messages(
    State(state): State<AppState>,
) -> Json<ListMessagesResponse> {
    let rows = sqlx::query("SELECT id, text, reply, future FROM messages")
        .fetch_all(&state.db)
        .await
        .unwrap();

    let items = rows
        .into_iter()
        .map(|row| MessageItem {
            id: row.get::<i64, _>("id") as u64,
            text: row.get::<String, _>("text"),
            reply: row.get::<String, _>("reply"),
            future: row.get::<String, _>("future"),
        })
        .collect();

    Json(ListMessagesResponse { items })
}

async fn create_daily_entry(
    State(state): State<AppState>,
    Json(req): Json<CreateDailyEntryRequest>,
) -> StatusCode {
    sqlx::query(
        "INSERT INTO daily_entries (action, mood, goal, reflection) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(req.action)
    .bind(req.mood)
    .bind(req.goal)
    .bind(req.reflection)
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::CREATED
}

async fn list_daily_entries(
    State(state): State<AppState>,
) -> Json<Vec<DailyEntry>> {
    let rows = sqlx::query(
        "SELECT id, action, mood, goal, reflection, created_at FROM daily_entries ORDER BY id DESC",
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    let items = rows
        .into_iter()
        .map(|row| DailyEntry {
            id: row.get::<i64, _>("id") as u64,
            action: row.get::<String, _>("action"),
            mood: row.get::<String, _>("mood"),
            goal: row.get::<String, _>("goal"),
            reflection: row.get::<String, _>("reflection"),
            created_at: row.get::<String, _>("created_at"),
        })
        .collect();

    Json(items)
}

fn generate_reply(text: &str, history: &[String], future: &str) -> String {
    let context = history.join(" / ");

    if context.contains("疲れ") && text.contains("頑張") {
        return "未来の自分から見ると、ちゃんと限界の中でも前に進んでたよ。その頑張りは無駄じゃなかった。".to_string();
    }

    if context.contains("不安") {
        return "あのとき感じていた不安、未来ではちゃんと乗り越えてるよ。だから今は焦らなくて大丈夫。".to_string();
    }

    if text.contains("おはよう") {
        return "未来の自分から見ると、その一日の始まりを大事にしてたのがすごく良かったよ。".to_string();
    }

    format!(
    "{}の自分から言うと、その悩み、ちゃんと乗り越えてたよ。だから今は大丈夫。",
    future
  )
}

#[tokio::main]
async fn main() {
    let db_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/futelt.db");

    std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();

    let connect_opts = SqliteConnectOptions::from_str(db_path.to_str().unwrap())
        .unwrap()
        .create_if_missing(true);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await
        .unwrap();

    // messages テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            reply TEXT NOT NULL,
            future TEXT NOT NULL,
            created_at DATETIME DEFAULT (datetime('now', 'localtime'))
        );
        "#,
    )
    .execute(&db)
    .await
    .unwrap();

    // daily_entries テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS daily_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            action TEXT NOT NULL,
            mood TEXT NOT NULL,
            goal TEXT NOT NULL,
            reflection TEXT NOT NULL,
            created_at DATETIME DEFAULT (datetime('now', 'localtime'))
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
        .route("/daily_entries", post(create_daily_entry).get(list_daily_entries))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3001";
    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}