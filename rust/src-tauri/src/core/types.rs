// Placeholder for future core types

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTransferProgress {
    pub filename: String,
    pub bytes_processed: u64,
    pub total_bytes: u64,
    pub status: TransferStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
}
