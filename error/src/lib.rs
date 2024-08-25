#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    DbError(sea_orm::error::DbErr),
    NetError(reqwest::Error),
    JsonError(serde_json::Error),
    RegexError(regex::Error),
    RuntimeError(String),
    Unreachable,
    Unsupported,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(ref err) => write!(f, "[IoError]: {}", err),
            Error::DbError(ref err) => write!(f, "[DbError]: {}", err),
            Error::NetError(ref err) => write!(f, "[NetError]: {}", err),
            Error::JsonError(ref err) => write!(f, "[JsonError]: {}", err),
            Error::RegexError(ref err) => write!(f, "[RegexError]: {}", err),
            Error::RuntimeError(ref err) => write!(f, "[RuntimeError]: {}", err),
            Error::Unreachable => write!(f, "[Unreachable]"),
            Error::Unsupported => write!(f, "[Unsupported]"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Error::IoError(ref e) => Some(e),
            Error::DbError(ref e) => Some(e),
            Error::NetError(ref e) => Some(e),
            Error::JsonError(ref e) => Some(e),
            Error::RegexError(ref e) => Some(e),
            Error::RuntimeError(_) => None,
            Error::Unreachable => None,
            Error::Unsupported => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<sea_orm::error::DbErr> for Error {
    fn from(value: sea_orm::error::DbErr) -> Self {
        Error::DbError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::NetError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonError(value)
    }
}

impl From<regex::Error> for Error {
    fn from(value: regex::Error) -> Self {
        Error::RegexError(value)
    }
}
