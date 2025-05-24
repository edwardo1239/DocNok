use tokio::fs;

use crate::models::errors::{FileError, FileErrorKind};


pub async fn open_file_as_text(path: &str ) -> Result<String, FileError>{    
    let contents = match fs::read_to_string(path).await {
        Ok(contents) => contents,
        Err(e) => {
            return Err(FileError::new(
                400,
                &format!("Failed to read file: {}", e),
                FileErrorKind::NotFound,
                "fs::reader::open_file_as_text"
            ));
        }
    };
    Ok("".to_string())
}