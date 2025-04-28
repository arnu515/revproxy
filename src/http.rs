use axum::{Router, response::Html, routing::get};

pub async fn run_app(listener: tokio::net::TcpListener) {
    let router = Router::new().route("/", get(index));
    axum::serve(listener, router).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
