use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

async fn home() -> Markup {
    html! {
        (header("พิสูจน์หลักฐานจังหวัดภูเก็ต"))
        h1 { "Hello, World!" }
        (footer())
    }
}

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (title) }
    }
}

fn footer() -> Markup {
    html! {}
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(home));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
