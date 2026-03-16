use crate::util::get_path;
use crate::{DATABASE, PREFIX};

use axum::{extract::Multipart, http::StatusCode};
use tokio::fs;

pub async fn upload(mut body: Multipart) -> Result<String, StatusCode> {
    let field = body
        .next_field()
        .await
        .unwrap()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let filename = field.file_name().unwrap_or("unamed").to_string();
    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

    let hash = blake3::hash(&data).to_hex().to_string();
    let path = get_path(&hash, true).await;

    fs::write(path, &data)
        .await
        .expect("couldn't write to file");

    let id = DATABASE.create_upload(
        hash.clone(),
        (data.len() / 1024) as i32,
        filename.to_string(),
    );

    Ok(format!("{PREFIX}{id}"))
}
