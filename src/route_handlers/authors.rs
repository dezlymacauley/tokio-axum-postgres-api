use axum::{routing::get, Router};

async fn get_authors() -> &'static str {
    "This is /authors"
}

async fn get_author_one() -> &'static str {
    "This is /authors/one"
}

async fn get_author_two() -> &'static str {
    "This is /authors/two"
}

pub fn authors_route_handlers() -> Router<()> {
    Router::new()
        .route("/", get(get_authors))
        .route("/one", get(get_author_one))
        .route("/two", get(get_author_two))
}
