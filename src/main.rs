// Imports the TcpListener struct from the tokio package.  
use tokio::net::TcpListener;

// Imports the `Router` struct from the axum package
use axum::Router;

// Declare the `route_handlers` directory as a module
mod route_handlers;

// Import the handler functions for the `/` route,
// and the `/authors` route.
use route_handlers::{root_route_handlers, authors_route_handlers};

// Defines the entry point of the server
#[tokio::main]
async fn main() {
    // Connection settings
    let protocol: &str = "http";
    let host: &str = "127.0.0.1";
    let port: u16 = 4666;
    let url: String = format!("{protocol}://{host}:{port}");

    // Creates a new instance of the `Router` struct
    // `Router<()>` means that this axum API does not share any global state
    // across its handler functions. This will change once
    // the Postgres database is added.
    // app needs to be mutable in order to add routes to it.
    // (Unless you add the routes during creation, with the builder pattern).
    // Then adds the routes to the axum router
    // Use .merge() for root-level routes 
    // and .nest() for prefixed sub-routers
    let app: Router<()> = Router::new()
        .merge(root_route_handlers())
        .nest("/authors", authors_route_handlers());

    // Sets up an asynchronus TCP connection that uses 
    // the `Connection settings`
    let tcp_listener = TcpListener::bind(format!("{host}:{port}"))
        .await
        .expect("Error: Could not establish a TCP connection");

    // Displays the connection settings
    println!("\n The server is running on:");
    println!("{url}\n");

    // Start the server
    // The `axum::serve` function requires to things:
    // 1. A tcp_lister
    // 2. A router
    // This is basically the equivalent of Bun.serve() except that you have
    // to create your own TCP listener, 
    // since Rust does not have a built-in async runtime like Bun does.
    axum::serve(tcp_listener, app)
        .await
        .expect("Failed to start the Axum Server");
}
