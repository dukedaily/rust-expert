use super::http::{Method, Request, Response, StatusCode};
use super::server1::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string())), // _ => Response::new(StatusCode::NotFound, None),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>HELLO</h1>".to_string())), // _ => Response::new(StatusCode::NotFound, None),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }

        // Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}
