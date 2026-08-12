/*
  ABOUT: src/mock-database/schema.rs

  The variable types that define the database.

*/

//_____________________________________________________________________________

// SECTION: Authors Table

// Each author in the table will be saved as an object
// that contains the following fields.
pub struct Author {
    pub id: String,
    pub name: String,
}

// The AuthorsTable is an array of Author objects.
pub type AuthorsTable = Vec<Author>;

//_____________________________________________________________________________

// SECTION: Database Schema

// The database is an array of tables
pub struct DatabaseSchema {
    // tableName: Data Type
    pub authors: AuthorsTable,
}

//_____________________________________________________________________________
