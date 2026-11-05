//axum -- express/gin
//tokio -- async runtime, executes asynchronous rust code
//tracing -- structured application logging
//tracing-subscriber -- collects and formats tracing events

use axum::{Router, routing::get};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //a result is either success or failure
    //ok() and Err() return krta Result
    //() is ok
    //the error side of the result is box wali puri cheez.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let app = Router::new().route("/health", get(health_check));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::info!("server up and runnin on 3000, also my first rust server");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "hie thereeeeee amaan only and amaan goat"
}
