use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    InvalidCommand(String),
    NoFileName,
    FileNameNotUnicode,
    FileDoesNotExist(PathBuf),
    IOError {
        context: String,
        error: std::io::Error
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidCommand(msg) => write!(f, "Invalid Command: {}", msg),
            AppError::NoFileName => write!(f, "Provided path has no file name, so nothing can be (un)hidden"),
            AppError::FileNameNotUnicode => write!(f, "File name is not in unicode, so cannot be processed"),
            AppError::IOError { context, error } => write!(f, "I/O error encountered while {}: {}", context, error),
            AppError::FileDoesNotExist(path) => write!(f, "the provided file path '{}' does not exist", path.display())
        }
    }
}

impl Error for AppError {}
