//axum -- express/gin
//tokio -- async runtime, executes asynchronous rust code
//tracing -- structured application logging
//tracing-subscriber -- collects and formats tracing events

// use axum::{Router, routing::get};
// use tracing_subscriber::EnvFilter;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     //a result is either success or failure
//     //ok() and Err() return krta Result
//     //() is ok
//     //the error side of the result is box wali puri cheez.
//     tracing_subscriber::fmt()
//         .with_env_filter(EnvFilter::new("info"))
//         .init();

//     let app = Router::new().route("/health", get(health_check));
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

//     tracing::info!("server up and runnin on 3000, also my first rust server");

//     axum::serve(listener, app).await?;
//     Ok(())
// }

// async fn health_check() -> &'static str {
//     "hie thereeeeee amaan only and amaan goat"
// }

mod errors; //new
mod handlers;
mod models;
mod state;

use axum::{
    Router,
    routing::{get, post},
};

use tracing_subscriber::EnvFilter; //controls which tracing events get emitted

use handlers::todos::{create_todo, delete_todo, get_todo, list_todos, update_todo};
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info")) //sets the minimum log level to info
        .init();

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/todos", post(create_todo).get(list_todos))
        .route("/todos/{id}", get(get_todo).patch(update_todo).delete(delete_todo))
        .with_state(AppState::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("server up and running on 3000");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "hie thereeeeee amaan only and amaan goat"
}

//now we can run the server with cargo run
//and we can test the endpoints with curl or postman
