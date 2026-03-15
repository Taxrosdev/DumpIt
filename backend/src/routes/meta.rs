use crate::util::get_upload_limit;

use std::sync::LazyLock;

// Upload limit in KB
static UPLOAD_LIMIT: LazyLock<usize> = LazyLock::new(|| get_upload_limit() * 1024);

pub async fn upload_limit() -> String {
    format!("{}", *UPLOAD_LIMIT)
}
