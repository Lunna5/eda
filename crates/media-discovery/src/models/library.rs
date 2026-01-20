use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use super::{Movie, Series};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LibraryContent {
    Movies(Vec<Movie>),
    Series(Vec<Series>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub id: String,
    pub name: String,
    pub root_path: PathBuf,
    pub content: LibraryContent,
}

impl Library {
    pub fn resolve_path(&self, relative_to_lib: &Path) -> PathBuf {
        self.root_path.join(relative_to_lib)
    }
}