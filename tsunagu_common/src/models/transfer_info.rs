use serde::{Deserialize, Serialize};
use super::{DeviceInfo, FileInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferInfo {
    id: String,
    sender: DeviceInfo,
    receiver: DeviceInfo,
    files: Vec<FileInfo>,
    status: TransferStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    InProgress(f32), // Progress percentage
    Completed,
    Failed(String),
}

impl TransferInfo {
    pub fn new(sender: DeviceInfo, receiver: DeviceInfo, files: Vec<FileInfo>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            sender,
            receiver,
            files,
            status: TransferStatus::Pending,
        }
    }

    // Add getters for fields and methods to update status
}
