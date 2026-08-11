// Declares `root.rs` and `authors.rs` as part of the `route_handlers` module
pub mod root;
pub mod authors;

// Makes the `route_handlers` function 
// from `src/route_handlers/root.rs`  usable from outside this directory
pub use root::root_route_handlers;

// Makes the `authors_route_handlers` function 
// from `src/route_handlers/authors.rs`  usable from outside this directory
pub use authors::authors_route_handlers;
