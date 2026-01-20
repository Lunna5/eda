use std::path::PathBuf;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub container: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub resolution: (u32, u32),
    pub bitrate: u64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaVersion {
    pub id: String,
    pub relative_path: PathBuf,
    pub size_bytes: u64,
    pub metadata: MediaMetadata,
}