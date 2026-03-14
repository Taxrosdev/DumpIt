use crate::util::get_path;

use axum::{extract::Path, http::StatusCode};
use tokio::fs;

pub async fn download(Path(hash): Path<String>) -> Result<Vec<u8>, StatusCode> {
    let path = get_path(&hash, false).await;
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let buf = fs::read(path).await.expect("could not read uploaded file");
    Ok(buf)
}
