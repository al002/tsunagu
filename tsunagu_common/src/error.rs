use std::net::AddrParseError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsunaguError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Transfer error: {0}")]
    Transfer(String),
    #[error("Device error: {0}")]
    Device(String),
    #[error("Discovery error: {0}")]
    Discovery(String),
    #[error("mDNS error: {0}")]
    Mdns(#[from] mdns_sd::Error),
    #[error("Address parse error: {0}")]
    AddrParse(#[from] AddrParseError),
}

