# Futelt（フューテルト）

**Futelt** は、**未来の自分と対話するための Rust フルスタック Web アプリケーション**です。  
大規模言語モデル（LLM）を使用せず、**過去の行動ログと状態遷移ルール**に基づいて  
「未来の自分」からの返答を生成します。

> *未来の自分は、すでに答えを知っている。*

---

## コンセプト

近年、多くの対話アプリは AI（LLM）に依存しています。  
Futelt はそれとは異なり、以下の方針で設計されています。

-  外部 AI / LLM API を使用しない  
-  ブラックボックスな振る舞いをしない  
-  すべての返答ロジックが説明可能  
-  同じ入力から同じ結果が得られる（再現性）

「未来の自分」は、次の要素から構成されます。

- 日々の行動ログ
- 内部状態（疲労・モチベーション・不安など）
- ルールベースの状態遷移
- 過去の類似状況の参照

---

## 仕組み

1. ユーザーが日々のログ（睡眠・気分・作業内容など）を記録  
2. バックエンドが内部状態を数式で推定  
3. 状態に応じたルールを選択  
4. 過去のログを参照しながら返答を生成  
5. 「未来の自分」からのメッセージとして表示  

すべての処理は Rust で実装されており、  
**透明性・テスト容易性・拡張性**を重視しています。

---

## 技術スタック

### フロントエンド（現在）
- HTML
- CSS
- JavaScript
- Axumテンプレート

### フロントエンド（移行予定）
- Rust + WebAssembly
- Leptos
- SPA形式（PWA対応予定）

### バックエンド
- Rust
- axum
- tokio（非同期ランタイム）
- tower-http（静的ファイル配信）

### データベース
- SQLite（開発用）
- PostgreSQL（公開運用想定）
- sqlx（マイグレーション対応予定）

---

## ディレクトリ構成

```text
futelt/
├─ .github/
│  └─ pull_request_template.md   # PRテンプレート
│
├─ crates/
│  ├─ domain/
│  │   ├─ src/
│  │   │   └─ lib.rs             # ドメインモデル
│  │   └─ Cargo.toml
│  │
│  └─ engine/
│      ├─ src/
│      └─ Cargo.toml             # 返答生成ロジック
│
├─ data/
│  └─ futelt.db                  # SQLiteデータベース
│
├─ services/
│  └─ api/
│      ├─ src/
│      │   └─ main.rs            # APIサーバー
│      ├─ templates/
│      │   └─ index.html         # HTMLテンプレート
│      ├─ static/
│      │   └─ assets/            # API側で配信する画像ファイル
│      └─ Cargo.toml
│
├─ web/
│  ├─ src/
│  │   ├─ app.rs                 # Leptosアプリ本体
│  │   ├─ lib.rs
│  │   └─ main.rs                # Leptosエントリーポイント
│  │
│  ├─ public/
│  │   ├─ assets/                # フロントエンド用画像
│  │   └─ favicon.ico
│  │
│  ├─ style/                     # CSS
│  ├─ end2end/                   # Playwrightテスト
│  └─ Cargo.toml
│
├─ Cargo.toml                    # Rust workspace設定
├─ Cargo.lock
├─ README.md
└─ LICENSE
```
