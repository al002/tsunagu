mod config;
mod cli;

use anyhow::Result;
use cli::{Cli, CliApp};
use clap::Parser;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let mut app = CliApp::new().await?;
    app.run(cli).await?;

    Ok(())
}
