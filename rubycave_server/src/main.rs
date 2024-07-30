use color_eyre::eyre;
use rpc::tcp::TcpServer;

mod rpc;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let server = TcpServer::new("0.0.0.0:1616").await?;
    server.run().await;

    Ok(())
}
