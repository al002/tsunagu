use super::Discovery;
use crate::error::TsunaguError;
use crate::models::DeviceInfo;
use crate::Result;
use async_trait::async_trait;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::str;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info};

const SERVICE_TYPE: &str = "_tsunagu._tcp.local.";

pub struct MdnsDiscovery {
    mdns: ServiceDaemon,
    discovered_devices: Arc<RwLock<HashMap<String, DeviceInfo>>>,
    local_device: DeviceInfo,
}

impl MdnsDiscovery {
    pub fn new(local_device: DeviceInfo) -> Result<Self> {
        let mdns = ServiceDaemon::new().map_err(TsunaguError::Mdns)?;
        Ok(Self {
            mdns,
            discovered_devices: Arc::new(RwLock::new(HashMap::new())),
            local_device,
        })
    }

    fn create_service_info(&self) -> ServiceInfo {
        let mut properties = HashMap::new();
        properties.insert("model".to_string(), self.local_device.model().to_string());
        properties.insert("os".to_string(), self.local_device.os().to_string());
        properties.insert(
            "version".to_string(),
            self.local_device.version().to_string(),
        );

        let instance_name = self.local_device.name();
        let hostname = format!("{}.", instance_name);

        ServiceInfo::new(
            SERVICE_TYPE,
            &instance_name,
            &hostname,
            self.local_device.ip(),
            self.local_device.port(),
            Some(properties),
        )
        .expect("Failed to create ServiceInfo")
    }

    async fn handle_event(&self, event: ServiceEvent) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let ip = info
                    .get_addresses()
                    .iter()
                    .next()
                    .map(|&addr| addr)
                    .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
                let model = info.get_property_val_str("model").unwrap_or_default();
                let os = info.get_property_val_str("os").unwrap_or_default();
                let version = info.get_property_val_str("version").unwrap_or_default();

                let hostname = info.get_hostname();

                let device_info = DeviceInfo::new(
                    hostname.to_string(),
                    model.to_string(),
                    ip.to_string(),
                    info.get_port(),
                    os.to_string(),
                    version.to_string(),
                );

                let mut devices = self.discovered_devices.write().await;
                if let Some(existing_device) = devices.get(&device_info.id().to_string()) {
                    if existing_device != &device_info {
                        info!("Updating existing device: {}", info.get_fullname());
                        devices.insert(device_info.id().to_string(), device_info);
                    } else {
                        debug!("Device already exists and is up-to-date: {}", info.get_fullname());
                    }
                } else {
                    info!("New device discovered: {}", info.get_fullname());
                    devices.insert(device_info.id().to_string(), device_info);
                }
            }
            ServiceEvent::ServiceRemoved(name, _type) => {
                debug!("Service removed: {} ({})", name, _type);
                self.discovered_devices
                    .write()
                    .await
                    .retain(|_, v| v.name() != name);
                info!("Device removed: {}", name);
            }
            _ => {}
        }
    }

    pub async fn manual_discover(&self) -> Result<()> {
        info!("Manually triggering device discovery");
        self.mdns.browse(SERVICE_TYPE).map_err(TsunaguError::Mdns)?;
        Ok(())
    }
}

#[async_trait]
impl Discovery for MdnsDiscovery {
    async fn start(&mut self) -> Result<()> {
        info!("Starting mDNS discovery");
        let browse_handle = self.mdns.browse(SERVICE_TYPE).map_err(TsunaguError::Mdns)?;
        let discovery = self.clone();

        tokio::spawn(async move {
            info!("mDNS discovery loop started");
            while let Ok(event) = browse_handle.recv_async().await {
                discovery.handle_event(event).await;
            }
            info!("mDNS discovery loop ended");
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping mDNS discovery");
        self.mdns.shutdown().map_err(TsunaguError::Mdns)?;

        Ok(())
    }

    async fn discover_devices(&self) -> Result<Vec<DeviceInfo>> {
        let devices = self
            .discovered_devices
            .read()
            .await
            .values()
            .cloned()
            .collect();
        info!("Discovered devices: {:?}", devices);
        Ok(devices)
    }

    async fn make_discoverable(&mut self, duration: Duration) -> Result<()> {
        info!("Making device discoverable for {:?}", duration);
        let service_info = self.create_service_info();
        self.mdns
            .register(service_info)
            .map_err(TsunaguError::Mdns)?;

        // let mdns = self.mdns.clone();
        // let service_fullname = self.local_device.name().to_string();
        //
        // tokio::spawn(async move {
        //     tokio::time::sleep(duration).await;
        //     if let Err(e) = mdns.unregister(&service_fullname) {
        //         error!("Failed to unregister service: {}", e);
        //     } else {
        //         info!("Service unregistered: {}", service_fullname);
        //     }
        // });

        Ok(())
    }
}

impl Clone for MdnsDiscovery {
    fn clone(&self) -> Self {
        Self {
            mdns: self.mdns.clone(),
            discovered_devices: Arc::clone(&self.discovered_devices),
            local_device: self.local_device.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_mdns_discovery() {
        let local_device = DeviceInfo::new(
            "TestDevice.local.".to_string(),
            "TestModel".to_string(),
            Ipv4Addr::LOCALHOST.to_string(),
            8000,
            "TestOS".to_string(),
            "1.0".to_string(),
        );

        let mut discovery = MdnsDiscovery::new(local_device).unwrap();

        discovery.start().await.unwrap();
        discovery
            .make_discoverable(Duration::from_secs(5))
            .await
            .unwrap();

        // Manually trigger discovery
        discovery.manual_discover().await.unwrap();

        // Wait for a moment to allow discovery
        tokio::time::sleep(Duration::from_secs(2)).await;

        let devices = discovery.discover_devices().await.unwrap();
        println!("Discovered devices: {:?}", devices);

        discovery.stop().await.unwrap();
    }
}
