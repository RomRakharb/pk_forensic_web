use axum::{response::Html, routing::get, Router};
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use tokio::fs;
use tower_http::services::ServeDir;

use pk_forensic_web::components::{carousel, header, nav_bar};

#[allow(dead_code)]
struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

fn home() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset = "utf-8";
                title { "พิสูจน์หลักฐานจังหวัดภูเก็ต" }
                script src = "https://cdn.tailwindcss.com";
                (PreEscaped("<script>https://cdn.tailwindcss.com</script>"))
                script src = "https://unpkg.com/htmx.org@2.0.3";
                (PreEscaped("<script>https://unpkg.com/htmx.org@2.0.3</script>"))
            }
            body {
                (header())
                (nav_bar())
                (carousel())
            }
        }
    }
}

async fn html_template() -> Html<String> {
    let html = home();

    let html_str = html.clone().into_string();
    fs::write("./static/index.html", &html_str).await.unwrap();

    Html(html_str)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(html_template))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
