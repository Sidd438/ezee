use super::method::METHOD;
use std::convert::TryFrom;
use std::convert::From;
use std::str;
use std::str::Utf8Error;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};

pub struct Request {
    method: METHOD,
    path: String,
    body: String,
    query_params: Option<String>
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;
    }
}

pub enum ParseError {
    InvalidEncoding,
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            self::InvalidEncoding => "Invalid Encoding",
            self::InvalidRequest => "Invalid Request",
            self::InvalidProtocol => "Invalid Protocol",
            self::InvalidMethod => "Invalid Method",
        }
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}


impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Error for ParseError {

}