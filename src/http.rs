use axum::{Router, routing::get};
use maud::{Markup, html};

pub async fn run_app(listener: tokio::net::TcpListener) {
    let router = Router::new().route("/", get(index));
    axum::serve(listener, router).await.unwrap();
}

async fn index() -> Markup {
    html! {
        h1 { "Hello, world!" }
    }
}
