use std::{env, path::Path};

use crate::models::errors::{InputPathError, InputPathErrorKind};


pub fn read_validate_args() -> Result<String, InputPathError> {
    let args: Vec<String> = env::args().collect();
    validate_args(&args)
}

fn validate_args(args: &[String]) -> Result<String, InputPathError> {
    if args.len() < 2 {
        return Err(InputPathError::new(
            400,
            "No input path provided",
            InputPathErrorKind::MissingArgument,
            "main::run",
        ));
    }

    let path = &args[1];
    if path.trim().is_empty() {
        return Err(InputPathError::new(
            400,
            "Input path is empty",
            InputPathErrorKind::InvalidInput,
            "main::run",
        ));
    }

    if !Path::new(path).exists() {
        return Err(InputPathError::new(
            400,
            "Path does not exist",
            InputPathErrorKind::InvalidInput,
            "main::run",
        ));
    }

    Ok(path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_argument() {
        let args = vec!["Program".to_string()];
        let result = validate_args(&args);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(*e.kind(), InputPathErrorKind::MissingArgument);
        }
    }

    #[test]
    fn test_empty_path() {
        let args = vec!["Program".to_string(), "".to_string()];
        let result = validate_args(&args);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(*e.kind(), InputPathErrorKind::InvalidInput);
        }
    }

    #[test]
    fn test_nonexistent_path() {
        let args = vec!["Program".to_string(), "nonexistent/path".to_string()];
        let result = validate_args(&args);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(*e.kind(), InputPathErrorKind::InvalidInput);
        }
    }

    #[test]
    fn test_valid_path() {
        // Usa el directorio actual que sabemos que existe
        let current_dir = ".";
        let args = vec!["Program".to_string(), current_dir.to_string()];
        let result = validate_args(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current_dir);
    }
}