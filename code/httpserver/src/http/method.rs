use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "DELETE" => Ok(Method::DELETE),
            "PUT" => Ok(Method::PUT),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
