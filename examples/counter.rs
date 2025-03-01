use axum::{handler::get, Router};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(cookies: Cookies) -> String {
    let visited = if let Some(cookie) = cookies.get("visited") {
        cookie.value().parse().ok().unwrap_or(0)
    } else {
        0
    };
    cookies.add(Cookie::new("visited", (visited + 1).to_string()));
    format!("You've been here {} times before", visited)
}
