/*
  ABOUT: src/route-handlers/authors.ts

  This file contains all the route handler functions for:
  http://127.0.0.1:4666/authors (including nested routes)

*/

//_____________________________________________________________________________

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
