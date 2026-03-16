use crate::DATABASE;
use crate::util::get_path;

use axum::{extract::Path, http::StatusCode};
use tokio::fs;

pub async fn download(Path(id): Path<String>) -> Result<Vec<u8>, StatusCode> {
    let upload = DATABASE.get_upload(&id).map_err(|e| {
        eprintln!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let upload = upload.ok_or(StatusCode::NOT_FOUND)?;

    let path = get_path(&upload.hash, false).await;
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let buf = fs::read(path).await.expect("could not read uploaded file");
    Ok(buf)
}
