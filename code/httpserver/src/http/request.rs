use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str;
use std::str::Utf8Error;
use super::{QueryString, QueryStringValue};

// pub struct Request {
//     path: String,
//     query_string: Option<String>,
//     method: Method,
// }

// this is lifie time which to associate the buffer with the request
// pub struct Request<'buf> {
//     path: &'buf str,
//     query_string: Option<&'buf str>,
//     method: Method,
// }

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, 
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str { 
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    // 我们只关心QueryString的信息，而不是Option的信息
    pub fn query_string(&self) -> Option<&QueryString> {
         self.query_string.as_ref()
    }
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Request, String>{}
// }

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // type Error = String;
    type Error = ParseError;

    // 'buf 是我们自己定义的lifetime
    // 在下面的代码块中，我们告诉了编译器：buf和返回值Request使用同一个lifetime
    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // 方式1：普通方式
        // match str::from_utf8(buf) {
        //     Ok(request) =>{}
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }

        // 方式2：更简洁（推荐）的方式
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // 方式3：特殊的语法
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        // 这个要与下面的impl From<Utf8Error> for ParseError {
        let request = str::from_utf8(buf)?;

        /*
         * match里面具体是的条件取决于业务函数的返回值:
         * 如果是Result：那么就是Ok和Err
         * 如果是Option：那么就是Some和None
         */

        /*  语法糖
         *  如果返回值是Resut，则可以直接使用or(err)?，而不必使用match
         *  如果返回值是Option，则可以直接使用ok_or(err)?，而不必使用match
         */

        // 方式1：
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        // 方式2：
        // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // 方式1：
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[i..];
        //     }
        //     None => {}
        // }

        // 方式2：
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[i..];
        // }

        // 方式3：推荐，更加简洁
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[i..];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })

    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    /* 太冗余的写法
    let mut iter = request.chars();
    loop {
        let item = iter.next();
        match item {
            Some(c) => {}
            None => break,
        }
    } */

    // 两个迭代器
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    unimplemented!()
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        let msg = match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        };

        return msg;
    }
}

// 为了上面的？而实现这个方法，用于解释Error转化问题
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// 为了上面的？而实现这个方法，用于解释Error转化问题
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
