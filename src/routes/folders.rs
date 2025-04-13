use poem::{get, Route};
use crate::handlers::files::{get_files, get_file, download_file};
pub fn file_routes() -> Route {
    Route::new()
        .at("/folders", get(get_files))
        .at("/folders/:path", get(get_file))
        .at("/folders/:path/download", get(download_file))
}