use crate::DATABASE;
use crate::util::get_path;

use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue, StatusCode, header},
};
use tokio::fs;

pub async fn download(Path(id): Path<String>) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let upload = DATABASE.get_upload(&id).map_err(|e| {
        eprintln!("{e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let upload = upload.ok_or(StatusCode::NOT_FOUND)?;

    let path = get_path(&upload.hash, false).await;
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Set Content-Disposition header so filenames are correct
    let mut headermap = HeaderMap::with_capacity(1);
    headermap.append(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", upload.filename)).unwrap(),
    );

    let buf = fs::read(path).await.expect("could not read uploaded file");
    Ok((headermap, buf))
}
