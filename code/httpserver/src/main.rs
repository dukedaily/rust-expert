// #![allow(dead_code)]

//use相当于import 文件名::对象名称
use server1::Server;
// use http::method::Method;
// use http::request::Request;
use http::Method;
use http::Request;
use website_handler::WebsiteHandler;

// 模块名，也就是文件名
mod server1;
mod website_handler;

// 在这里使用mod相当于把代码拷贝了一份过来
// 只有使用mod引入之后，在上面才能使用use关键字
mod http;

fn main() {
    let method = Method::GET;
    // dbg!(method);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}
