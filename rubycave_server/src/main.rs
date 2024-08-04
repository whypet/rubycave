use color_eyre::eyre::{self, eyre};
use game::Game;
use tracing::info;

mod game;
mod rpc;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    info!("starting");

    let game = Game::new().await?;
    game.run().await.ok_or(eyre!("failed to run game server"))?;

    Ok(())
}
