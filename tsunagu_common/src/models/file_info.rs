use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    name: String,
    size: u64,
    mime_type: String,
    last_modified: u64,
}

impl FileInfo {
    pub fn new(name: String, size: u64, mime_type: String, last_modified: u64) -> Self {
        Self {
            name,
            size,
            mime_type,
            last_modified,
        }
    }

    // Add getters for fields
}
