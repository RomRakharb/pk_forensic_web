use loco_rs::cli;
use pk_forensic_web::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App>().await
}
