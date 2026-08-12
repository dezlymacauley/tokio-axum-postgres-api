/*
  ABOUT: src/mock-database/seed_data_authors_table.rs

  Initial seed data for the authors table of the mock-database
*/

// Imports the Author type and AuthorsTable type
use super::schema::{Author, AuthorsTable};

// This is a function that will create a new instance of an `AuthorsTable`,
// that already contains initial data.
// The pub keyword allows this function to be used outside of this file
pub fn seedAuthorsTable() -> AuthorsTable {
    vec![
        Author {
            id: String::from("5ed614ec-b3cb-4b37-9f19-4304e5574fd5"),
            name: String::from("Seth Baradock")
        },
        Author {
            id: String::from("2221a287-b633-4473-950b-ba4e5b6e6632"),
            name: String::from("Cassie Elmore")
        }
    ]
}
