use std::{env, path::Path};

use crate::models::errors::{InputPathError, InputPathErrorKind};


pub fn read_validate_args() -> Result<String, InputPathError> {
    let args: Vec<String> = env::args().collect();
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
