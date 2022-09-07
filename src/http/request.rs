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
        let (method, request) = gen_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = gen_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = gen_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol)
        }

        let method : METHOD = method.parse()?


    }
}

fn gen_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
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
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl Error for ParseError {

}