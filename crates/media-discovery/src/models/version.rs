use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use eda_ffprobe::{FfProbe};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaVersion {
    pub id: String,
    pub relative_path: PathBuf,
    pub size_bytes: u64,
    pub metadata: FfProbe,
}