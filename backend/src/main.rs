pub mod routes;
pub mod util;
use routes::{download::download, upload::upload};
use util::get_upload_limit;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use tower_http::{
    limit::RequestBodyLimitLayer,
};

pub static UPLOAD_PATH: &str = "./upload";
pub static PREFIX: &str = "/api/download/";

#[tokio::main]
async fn main() {
    let upload_limit = get_upload_limit();

    let app = Router::new()
        .route("/upload", post(upload))
        .route("/download/{hash}", get(download))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(upload_limit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("could not bind to port 3000");
    axum::serve(listener, app)
        .await
        .expect("axum error, could not serve")
}
