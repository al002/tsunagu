use std::time::Duration;

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::info;
use tsunagu_common::{device::DeviceManager, discovery::{Discovery, MdnsDiscovery}, models::DeviceInfo, transfer::TcpFileTransfer};
use crate::config::CliConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the Tsunagu service
    Start,
    Discover {
        #[arg(short, long, default_value = "5")]
        timeout: u64,
    },
    /// Send a file
    Send {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        receiver: String,
    },
    /// Receive a file
    Receive {
        #[arg(short, long)]
        sender: String,
    },
}

pub struct CliApp {
    config: CliConfig,
    device_manager: DeviceManager,
    discovery: MdnsDiscovery,
    transfer: TcpFileTransfer,
}

impl CliApp {
    pub async fn new() -> Result<Self> {
        let config = CliConfig::load()?;
        let device_manager = DeviceManager::new().await?;
        let local_device = device_manager.get_current_device_info().await;

        info!("local device {:?}", local_device);

        let discovery = MdnsDiscovery::new(local_device.clone())?;
        let transfer = TcpFileTransfer::new(
            device_manager.get_current_device_info().await,
            config.download_dir.clone(),
        );

        Ok(Self {
            config,
            device_manager,
            discovery,
            transfer,
        })
    }

    pub async fn run(&mut self, cli: Cli) -> Result<()> {
        match cli.command {
            Some(Commands::Start) => self.start_service().await?,
            Some(Commands::Discover { timeout }) => self.discover_devices(timeout).await?,
            Some(Commands::Send { file, receiver }) => self.send_file(&file, &receiver).await?,
            Some(Commands::Receive { sender }) => self.receive_file(&sender).await?,
            None => self.start_service().await?,
        }

        Ok(())
    }

    async fn start_service(&mut self) -> Result<()> {
        info!("Starting Tsunagu service...");
        self.discovery.start().await?;

        self.discovery.make_discoverable(Duration::from_secs(3600)).await?; // Make discoverable for 1 hour
        let local_device = self.device_manager.get_current_device_info().await;
        info!("Tsunagu service started on port {}. Press Ctrl+C to stop.", local_device.port());

        tokio::signal::ctrl_c().await?;
        info!("Stopping Tsunagu service...");
        self.discovery.stop().await?;
        Ok(())
    }

    async fn discover_devices(&mut self, timeout: u64) -> Result<()> {
        info!("Discovering devices for {} seconds...", timeout);
        self.discovery.start().await?;
        self.discovery.make_discoverable(Duration::from_secs(timeout)).await?;
        
        tokio::time::sleep(Duration::from_secs(timeout)).await;
        
        let devices = self.discovery.discover_devices().await?;
        info!("Discovered devices:");
        for device in devices {
            info!("- {} ({})", device.name(), device.ip());
        }
        
        self.discovery.stop().await?;
        Ok(())
    }

    async fn send_file(&self, file: &str, receiver: &str) -> Result<()> {
        info!("Sending file {} to {}", file, receiver);
        // Implement file sending logic here
        Ok(())
    }

    async fn receive_file(&self, sender: &str) -> Result<()> {
        info!("Receiving file from {}", sender);
        // Implement file receiving logic here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_app_creation() {
        let app = CliApp::new().await;
        assert!(app.is_ok());
    }

    #[tokio::test]
    async fn test_start_service() {
        let mut app = CliApp::new().await.unwrap();
        let cli = Cli {
            command: Some(Commands::Start),
        };
        let result = app.run(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_file() {
        let mut app = CliApp::new().await.unwrap();
        let cli = Cli {
            command: Some(Commands::Send {
                file: "test.txt".to_string(),
                receiver: "ReceiverDevice".to_string(),
            }),
        };
        let result = app.run(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_receive_file() {
        let mut app = CliApp::new().await.unwrap();
        let cli = Cli {
            command: Some(Commands::Receive {
                sender: "SenderDevice".to_string(),
            }),
        };
        let result = app.run(cli).await;
        assert!(result.is_ok());
    }
}

