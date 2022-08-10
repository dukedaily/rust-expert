// use std::convert::TryFrom;
// use std::convert::TryInto;
use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to convert request : {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // 无限循环
        loop {
            // 匹配
            match listener.accept() {
                // 成功，自动结构出变量，箭头函数处理逻辑，_表示忽略
                Ok((mut stream, socket)) => {
                    println!("socket: {}", socket);
                    let mut buffer: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request : {}", String::from_utf8_lossy(&buffer));
                            // 两种其他写法：
                            // Request::try_from(&buffer[..]);
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                            let response = match Request::try_from(&buffer as &[u8]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    handler.handle_request(&request)
                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>IT WORKS!</h1>".to_string()),
                                    // )
                                    // write!(stream, "{}", response);
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response : {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                // 失败
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
