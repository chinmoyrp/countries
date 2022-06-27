use std::net::SocketAddr;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    backend::start_server(addr).await?;

    Ok(())
}