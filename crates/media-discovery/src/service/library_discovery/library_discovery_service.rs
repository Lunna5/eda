use crate::{LibraryContent, LibraryContentKind, Movie, Series};

pub trait MovieDiscovery {
    fn discover_movies(&self) -> Vec<Movie>;
}

pub trait SeriesDiscovery {
    fn discover_series(&self) -> Vec<Series>;
}

pub trait LibraryDiscoveryService:
MovieDiscovery + SeriesDiscovery
{
    fn discover(
        &self,
        kind: LibraryContentKind,
    ) -> LibraryContent {
        match kind {
            LibraryContentKind::Movies => {
                LibraryContent::Movies(self.discover_movies())
            }
            LibraryContentKind::Series => {
                LibraryContent::Series(self.discover_series())
            }
        }
    }
}
