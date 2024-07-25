use color_eyre::eyre;
use tcp::TcpServer;

mod tcp;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let server = TcpServer::new("0.0.0.0:1616").await?;
    server.run().await?;

    Ok(())
}
