use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NetworkError(reqwest::Error),
    ParseError(serde_json::Error),
    RpcError { code: i32, message: String },
    ResponseError(String),
    NotFound(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
            Self::RpcError { code, message } => write!(f, "RPC error {}: {}", code, message),
            Self::ResponseError(msg) => write!(f, "Response error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NetworkError(e) => Some(e),
            Self::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::NetworkError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError(err)
    }
}