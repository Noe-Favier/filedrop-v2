use poem::{get, EndpointExt, Route};
use crate::handlers::files::{get_files, get_file, download_file};
use crate::middlewares::path_traversal_guard::PathTraversalGuard;

pub fn file_routes() -> Route {
    Route::new()
        .at("/files", get(get_files))
        .at("/files/:path", get(get_file))
        .at("/files/:path/download", get(download_file))
}