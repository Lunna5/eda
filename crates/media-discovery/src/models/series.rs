use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::models::version::MediaVersion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub episode_number: u32,
    pub relative_path: PathBuf,
    pub versions: Vec<MediaVersion>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: String,
    pub season_number: u32,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: String,
    pub title: String,
    pub release_year: Option<u16>,
    pub seasons: Vec<Season>,
}