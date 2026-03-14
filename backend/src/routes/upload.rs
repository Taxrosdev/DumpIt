use crate::PREFIX;
use crate::util::get_path;

use axum::{extract::Multipart, http::StatusCode};
use tokio::fs;

pub async fn upload(mut body: Multipart) -> Result<String, StatusCode> {
    let field = body
        .next_field()
        .await
        .unwrap()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

    let hash = blake3::hash(&data).to_hex().to_string();
    let path = get_path(&hash, true).await;

    fs::write(path, data).await.expect("couldn't write to file");

    Ok(format!("{PREFIX}{hash}"))
}
