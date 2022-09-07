#[derive(Debug)]
pub enum METHOD {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

use std::str::FromStr;


impl FromStr for METHOD {
    type Err = MethodError;

    fn from_str(meth: &str) -> Result<Self, Self::Err> {
        match meth {
            "GET"=>Ok(Self::GET),
            "POST"=>Ok(Self::POST),
            "PUT"=>Ok(Self::PUT),
            "PATCH"=>Ok(Self::PATCH),
            "DELETE"=>Ok(Self::DELETE),
            _=>Err(MethodError)
        }
    }
}