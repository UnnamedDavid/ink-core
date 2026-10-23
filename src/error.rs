use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
// Possible failures for the current ink engine
pub enum InkError {
    // File read/write error. Eg. Not found, permission denied
    Io(io::Error),
    // Poorly formatted JSON or schema mismatch
    Serialization(serde_json::Error),
    // Attempted operations that require strokes on an empty canvas
    EmptyCanvas,
    // Requested stroke index does not exist
    StrokeNotFound(usize),
}

// For human users reading end-user error messages
impl fmt::Display for InkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InkError::Io(err) => write!(f, "I/O error: {err}"),
            InkError::Serialization(err) => write!(f, "JSON formatting error: {err}"),
            InkError::EmptyCanvas => write!(f, "Canvas contains no strokes"),
            InkError::StrokeNotFound(i) => write!(f, "Stroke index {i} does not exist"),
        }
    }
}

// Allow automatic third party error directing to InkError
impl error::Error for InkError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            InkError::Io(err) => Some(err),
            InkError::Serialization(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for InkError {
    fn from(err: io::Error) -> Self {
        InkError::Io(err)
    }
}

impl From<serde_json::Error> for InkError {
    fn from(err: serde_json::Error) -> Self {
        InkError::Serialization(err)
    }
}
