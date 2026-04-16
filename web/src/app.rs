use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MessageItem {
    id: u64,
    text: String,
    reply: String,
    future: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ListResponse {
    items: Vec<MessageItem>,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let messages = LocalResource::new( move || async move {
            let res = Request::get("http://127.0.0.1:3002/messages")
                .send()
                .await
                .unwrap()
                .json::<ListResponse>()
                .await
                .unwrap();

            res.items
        },
    );

    view! {
        <main>
            <MetaTags />
            <h1>"Futelt (Leptos)"</h1>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    messages.get().map(|items: Vec<MessageItem>| {
                        view! {
                            <ul>
                                {items.into_iter().map(|m| {
                                    view! {
                                        <li>
                                            <p><strong>"あなた: "</strong>{m.text}</p>
                                            <p><strong>"未来("</strong>{m.future}<strong>"): "</strong>{m.reply}</p>
                                        </li>
                                    }
                                }).collect_view()}
                            </ul>
                        }
                    })
                }}
            </Suspense>
        </main>
    }
}