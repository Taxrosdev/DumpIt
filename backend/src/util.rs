use crate::UPLOAD_PATH;

use std::{path::PathBuf, env::var};
use tokio::fs;

pub async fn get_path(hash: &str, create_parent: bool) -> PathBuf {
    let parent = PathBuf::from(UPLOAD_PATH).join(&hash[..2]);

    if create_parent {
        fs::create_dir_all(&parent)
            .await
            .expect("could not create parent directory. Permissions?");
    }

    parent.join(&hash[2..])
}

/// This will return in MB the amount of data that can be uploaded at once.
pub fn get_upload_limit() -> usize {
    let env = match var("UPLOAD_LIMIT") {
        Ok(env) => env, 
        Err(_) => panic!("UPLOAD_LIMIT is not specified."),
    };
    
    let raw_upload_limit: usize = 
        env.parse().expect("UPLOAD_LIMIT is not a number. It should be the max MB amount that can be uploaded per file.");

    raw_upload_limit * 1000 * 1000
}

pub fn get_upload_directory() -> PathBuf {
    let env = match var("UPLOAD_PATH") {
        Ok(env) => env, 
        Err(_) => panic!("UPLOAD_PATH is not specified."),
    };
    
    PathBuf::from(env)
}
