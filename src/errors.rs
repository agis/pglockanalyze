use std::fmt;

#[derive(Debug)]
pub enum Error {
    ConfigParseError(postgres::Error),
    ConfigOtherError(String),
    AnalyzeError(pg_query::Error),
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Error {
        Error::ConfigParseError(e)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::ConfigOtherError(s.to_string())
    }
}

impl From<pg_query::Error> for Error {
    fn from(e: pg_query::Error) -> Error {
        Error::AnalyzeError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigParseError(e) => write!(f, "configuration error: {}", e),
            Self::ConfigOtherError(s) => write!(f, "configuration error: {}", s),
            Self::AnalyzeError(e) => write!(f, "analysis error: {}", e),
        }
    }
}
