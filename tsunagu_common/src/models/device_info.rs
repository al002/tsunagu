use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfo {
    id: String,
    name: String,
    model: String,
    ip: String,
    port: u16,
    os: String,
    version: String,
}

impl DeviceInfo {
    pub fn new(
        name: String,
        model: String,
        ip: String,
        port: u16,
        os: String,
        version: String,
    ) -> Self {
        // if !name.ends_with(".local.") {
        //     name = format!("{}.local.", name.trim_end_matches("."));
        // }

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            model,
            ip,
            port,
            os,
            version,
        }
    }

    pub fn new_from_mdns(
        name: String,
        model: String,
        ip: Option<String>,
        port: u16,
        os: String,
        version: String
    ) -> Option<Self> {
        ip.map(|ip_str| Self::new(name, model, ip_str, port, os, version))
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn ip(&self) -> &str {
        &self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn os(&self) -> &str {
        &self.os
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}
