use crate::{Movie, Series};
use crate::service::library_discovery::library_discovery_service::{LibraryDiscoveryService, MovieDiscovery, SeriesDiscovery};

struct FsLibraryDiscoveryService;

impl MovieDiscovery for FsLibraryDiscoveryService {
    fn discover_movies(&self) -> Vec<Movie> {
        todo!()
    }
}

impl SeriesDiscovery for FsLibraryDiscoveryService {
    fn discover_series(&self) -> Vec<Series> {
        todo!()
    }
}

impl<T> LibraryDiscoveryService for T
where
    T: MovieDiscovery + SeriesDiscovery,
{}
