use axum::{routing::get, Router};
use pk_forensic_web::pages::{home, news};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/news", get(news))
        .nest_service("/static", ServeDir::new("static"));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
