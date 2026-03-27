use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // 1. Build our application with a single route
    let app = Router::new().route("/", get(handler));

    // 2. Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}

// A simple handler that returns a string
async fn handler() -> &'static str {
    "Hello from Rust and Axum!"
}