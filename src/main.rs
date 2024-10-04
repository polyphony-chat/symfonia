use std::time::Duration;

use clap::Parser;
use symfonia::Args;

#[tokio::main]
pub(crate) async fn main() -> Result<(), symfonia::errors::Error> {
    let args = Args::parse();
    let server = symfonia::Server::new(args).await?;
    server
        .start()
        .await
        .map_err(|e| symfonia::errors::Error::Custom(e.to_string()))
}
