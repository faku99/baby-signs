use baby_signs::app::App;
use loco_rs::cli;
use migration::Migrator;

#[tokio::main]
async fn main() -> loco_rs::Result<(), Box<loco_rs::Error>> {
    Ok(cli::main::<App, Migrator>().await?)
}
