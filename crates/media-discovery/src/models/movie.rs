use serde::{Deserialize, Serialize};
use crate::models::version::MediaVersion;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub release_year: Option<u16>,
    pub versions: Vec<MediaVersion>,
}