pub mod db;
pub mod routes;
pub mod util;

use routes::{download::download, meta, upload::upload};
use std::sync::LazyLock;
use tokio::signal;
use util::get_upload_limit;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use tower_http::limit::RequestBodyLimitLayer;

pub static UPLOAD_PATH: &str = "./upload";
pub static PREFIX: &str = "/api/download/";
pub static DATABASE: LazyLock<db::Database> = LazyLock::new(db::Database::init);

#[tokio::main]
async fn main() {
    // In bytes
    let upload_limit = get_upload_limit() * 1024 * 1024;

    tokio::spawn(async move { cleanup_timer().await });

    let app = Router::new()
        .route("/upload", post(upload))
        .route("/download/{hash}", get(download))
        .route("/meta/upload_limit", get(meta::upload_limit))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(upload_limit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("could not bind to port 3000");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("axum error, could not serve")
}

#[cold]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

async fn cleanup_timer() {
    use tokio::time::{MissedTickBehavior, interval};

    let mut interval = interval(std::time::Duration::from_hours(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    loop {
        interval.tick().await;
        DATABASE.cleanup().await;
    }
}
