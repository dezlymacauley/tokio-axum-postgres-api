/*
  ABOUT: src/route_handlers/authors.ts

  This file contains all the route handler functions for:
  http://127.0.0.1:4666/authors (including nested routes)

*/

//_____________________________________________________________________________

use axum::{routing::get, Router};

//_____________________________________________________________________________

// SECTION: In-memory database

struct Author {
    id: String,
    name: String
}

let authorsDatabase: Vec<Author> = vec![
  // {
  //   id: "5ed614ec-b3cb-4b37-9f19-4304e5574fd5",
  //   name: "Seth Baradock"
  // },
];

/*

type Author = {
  id: string;
  name: string;
};

const authorsDatabase: Array<Author> = [
  {
    id: "5ed614ec-b3cb-4b37-9f19-4304e5574fd5",
    name: "Seth Baradock"
  },
  {
    id: "2221a287-b633-4473-950b-ba4e5b6e6632",
    name: "Cassie Elmore"
  }
];

*/

//_____________________________________________________________________________

async fn get_authors() -> &'static str {
    "This is /authors"
}

// async fn get_author_one() -> &'static str {
//     "This is /authors/one"
// }

pub fn authors_route_handlers() -> Router<()> {
    Router::new()
        .route("/", get(get_authors))
        // .route("/one", get(get_author_one))
}
