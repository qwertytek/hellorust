use loco_rs::cli;
use helloworld::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App>().await
}
