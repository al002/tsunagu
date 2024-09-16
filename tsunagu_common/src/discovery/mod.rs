use async_trait::async_trait;
use crate::models::DeviceInfo;
use crate::Result;

mod mdns_discovery;
pub use mdns_discovery::MdnsDiscovery;

#[async_trait]
pub trait Discovery {
    /// Start the discovery service
    async fn start(&mut self) -> Result<()>;

    /// Stop the discovery service
    async fn stop(&mut self) -> Result<()>;

    /// Discover nearby devices
    async fn discover_devices(&self) -> Result<Vec<DeviceInfo>>;

    /// Make the current device discoverable
    async fn make_discoverable(&mut self, duration: std::time::Duration) -> Result<()>;
}


