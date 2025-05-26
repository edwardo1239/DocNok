use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum InputPathErrorKind {
    InvalidInput,
    MissingArgument,
}

#[derive(Debug)]
pub struct InputPathError {
    code: i32,
    message: String,
    kind: InputPathErrorKind,
    location: String,
}

impl InputPathError {
    pub fn new(code: i32, message: &str, kind: InputPathErrorKind, location: &str) -> Self {
        InputPathError {
            code,
            message: message.to_string(),
            kind,
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &InputPathErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn location(&self) -> &str {
        &self.location
    }
}

impl fmt::Display for InputPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} at {}", self.code, self.message, self.location)
    }
}

impl Error for InputPathError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

//#region FileError
#[derive(Debug, PartialEq, Eq)]
pub enum FileErrorKind {
    NotFound,
    PermissionDenied,
    InvalidFormat,
    EmptyFile,
    Other,
}

#[derive(Debug)]
pub struct FileError {
    code: i32,
    message: String,
    kind: FileErrorKind,
    location: String,
}

impl FileError {
    pub fn new(code: i32, message: &str, kind: FileErrorKind, location: &str) -> Self {
        FileError {
            code,
            message: message.to_string(),
            kind,
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &FileErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn location(&self) -> &str {
        &self.location
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} at {}", self.code, self.message, self.location)
    }
}

impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

//#region ProcessError
#[derive(Debug, PartialEq, Eq)]
pub enum ProcessErrorKind {
    EmptyInput,
    SyntaxysError,
    MissingSection,
    InvalidFormat,
    UnsupportedFeature,
}

#[derive(Debug)]
pub struct ProcessError {
    code: i32,
    message: String,
    kind: ProcessErrorKind,
    location: String,
}

impl ProcessError {
    pub fn new(code: i32, message: &str, kind: ProcessErrorKind, location: &str) -> Self {
        ProcessError {
            code,
            message: message.to_string(),
            kind,
            location: location.to_string(),
        }
    }

    pub fn kind(&self) -> &ProcessErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> i32 {
        self.code
    }
    pub fn location(&self) -> &str {
        &self.location
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {} at {}", self.code, self.message, self.location)
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}