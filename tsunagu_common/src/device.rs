use local_ip_address::local_ip;
use std::net::IpAddr;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::env;
use gethostname::gethostname;

use crate::models::DeviceInfo;
use crate::Result;
use crate::error::TsunaguError;

pub struct DeviceManager {
    device_info: Arc<RwLock<DeviceInfo>>,
}

impl DeviceManager {
    pub async fn new() -> Result<Self> {
        let os = env::consts::OS.to_string();
        let version = env::consts::ARCH.to_string();
        let hostname = Self::get_hostname()?;

        let device_info = DeviceInfo::new(
            hostname,
            std::env::consts::OS.to_string(),
            Self::get_local_ip()?.to_string(),
            0, // Port will be set later when starting the server
            os,
            version,
        );

        Ok(Self {
            device_info: Arc::new(RwLock::new(device_info)),
        })
    }

    /// Get the current device's information
    pub async fn get_current_device_info(&self) -> DeviceInfo {
        self.device_info.read().await.clone()
    }

    /// Update the current device's information
    pub async fn update_device_info(&self, info: DeviceInfo) -> Result<()> {
        let mut device_info = self.device_info.write().await;
        *device_info = info;
        Ok(())
    }

    /// Update the device's port
    pub async fn update_port(&self, port: u16) -> Result<()> {
        let mut device_info = self.device_info.write().await;
        device_info.set_port(port);
        Ok(())
    }

    /// Get the local IP address
    fn get_local_ip() -> Result<IpAddr> {
        local_ip().map_err(|e| TsunaguError::Network(format!("Failed to get local IP: {}", e)))
    }

    /// Get the hostname
    fn get_hostname() -> Result<String> {
        gethostname().to_str()
            .ok_or_else(|| TsunaguError::Device("Failed to get hostname".into()))
            .map(|s| s.to_string())
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_manager_creation() -> Result<()> {
        let manager = DeviceManager::new().await?;
        let info = manager.get_current_device_info().await;

        assert!(!info.name().is_empty());
        assert!(!info.model().is_empty());
        assert!(!info.ip().is_empty());
        assert_eq!(info.port(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_device_info_update() -> Result<()> {
        let manager = DeviceManager::new().await?;

        // Test hostname
        let hostname = DeviceManager::get_hostname()?;
        assert!(!hostname.is_empty());

        // Test local IP
        let local_ip = DeviceManager::get_local_ip()?;
        assert!(local_ip.is_ipv4() || local_ip.is_ipv6());

        // Test updating port
        manager.update_port(8000).await.unwrap();
        let updated_info = manager.get_current_device_info().await;
        assert_eq!(updated_info.port(), 8000);

        Ok(())
    }

    #[tokio::test]
    async fn test_port_update() -> Result<()> {
        let manager = DeviceManager::new().await?;

        manager.update_port(9000).await?;
        let port_updated_info = manager.get_current_device_info().await;

        assert_eq!(port_updated_info.port(), 9000);

        Ok(())
    }

    #[test]
    fn test_get_hostname() {
        let hostname = DeviceManager::get_hostname().unwrap();
        assert!(!hostname.is_empty());
    }

    #[test]
    fn test_get_local_ip() {
        let ip = DeviceManager::get_local_ip().unwrap();
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }
}
