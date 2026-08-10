// Imports the `Router` struct from the axum package,
// and the `get` function for making GET requests
use axum::{routing::get, Router};

// Imports the TcpListener struct from the tokio package.  
use tokio::net::TcpListener;

// Defines the entry point of the server

#[tokio::main]
async fn main() {
    // Connection settings
    let protocol: &str = "http";
    let host: &str = "127.0.0.1";
    let port: u16 = 7878;
    let url: String = format!("{protocol}://{host}:{port}");

    // Creates a new instance of the `Router` struct
    // `Router<()>` means that this axum API does not share any global state
    // across its handler functions. This will change once
    // the Postgres database is added.
    // app needs to be mutable in order to add routes to it.
    // (Unless you add the routes during creation, with the builder pattern).
    let mut app: Router<()> = Router::new();

    // Sets up a basic route that returns text data
    app = app.route("/", get(|| async {"Hello Axum!"}));

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
        .expect("Failed to start TCP router");
}
