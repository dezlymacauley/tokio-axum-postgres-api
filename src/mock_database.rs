/*
  ABOUT: src/mock_database.rs

  The entry point for the mock database.
*/

// Declares `schema.rs` and `seed_data_authors_table.rs` 
// as part of the `mock_database` module
pub mod schema;
pub mod seed_data_authors_table;

// Import the structure of the database
use schema::DatabaseSchema;

// Import the seedAuthorsTable function
use seed_data_authors_table::seedAuthorsTable;

// Create a new mock database (which follows the structure of DatabaseSchema)
// Create a mock database function, that creates a new database that follows
// the structure of the DatabaseSchema struct.
pub fn create_mock_database() -> DatabaseSchema {
    DatabaseSchema {
        // The `seedAuthorsTable` function is used to create a new authors
        // table that already has data
        authors: seedAuthorsTable()
    }
}

//_____________________________________________________________________________

// TODO: Figure out how the exports will work

// Allow the mock database to be used outside this file
// export { mock_database };

// allow the data types that were defined in src/mock_database/schema.rs
// to be used outside of this file.
// Usage: `crate::mock_database`
pub use schema::{Author, AuthorsTable, DatabaseSchema};

//_____________________________________________________________________________
