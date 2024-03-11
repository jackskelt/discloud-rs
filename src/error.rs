use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    InvalidToken,
    Ratelimited,
    ServerError,
    Forbidden,
    InvalidRequest(&'static str),
    NotFound,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Reqwest(ref e) => write!(f, "HTTP request error: {e}"),
            Error::InvalidToken => write!(f, "Invalid API token provided"),
            Error::Ratelimited => write!(f, "You have been ratelimited"),
            Error::InvalidRequest(r) => write!(f, "Invalid request: {r}"),
            Error::ServerError => write!(f, "Server error"),
            Error::Forbidden => write!(f, "Forbidden access"),
            Error::NotFound => write!(f, "Not found"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}
