// Imports the `Router` struct from the axum package,
// and the `get` function for making GET requests
use axum::{Router, routing::get};

async fn get_root() -> &'static str {
    "This is /"
}

pub fn root_route_handlers() -> Router<()> {
    Router::new().route("/", get(get_root))
}
