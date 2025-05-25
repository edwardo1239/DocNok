use tokio::fs;

use crate::models::errors::{FileError, FileErrorKind};

pub async fn open_file_as_text(path: &str) -> Result<String, FileError> {
    let contents = match fs::read_to_string(path).await {
        Ok(contents) => contents,
        Err(e) => {
            return Err(FileError::new(
                400,
                &format!("Failed to read file: {}", e),
                FileErrorKind::NotFound,
                "fs::reader::open_file_as_text",
            ));
        }
    };
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_file_as_text() {
        let path = "src/mock/test.txt"; // <-- sin el "./"
        let contents = open_file_as_text(path).await;
        assert!(contents.is_ok());
        assert_eq!(contents.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn test_open_file_as_text_file_not_found() {
        let path = "src/mock/nonexistent.txt";
        let contents = open_file_as_text(path).await;
        assert!(contents.is_err());
        if let Err(e) = contents {
            assert_eq!(*e.kind(), FileErrorKind::NotFound);
        }
    }
}
