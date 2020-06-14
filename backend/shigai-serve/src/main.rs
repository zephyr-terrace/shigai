use std::io::prelude::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;

const SRC_ROOT: &str = "../../frontend/src";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];

    if let Ok(buf_length) = stream.read(&mut buffer) {
        let message = String::from_utf8_lossy(&buffer[..buf_length]).to_string();
        println!("{}", message);

        let http_req: http::HTTPRequest = http::process_request(message);
        router::route(http_req, stream);
    } else {
        let response = "HTTP/1.1 400 Bad Request \r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

mod router {
    use crate::http::*;
    use std::fs;
    use std::io::prelude::*;
    use std::net::TcpStream;

    pub fn route(req: HTTPRequest, stream: TcpStream) {
        match req {
            HTTPRequest {
                verb: HTTPVerb::GET,
                route: rte,
                ..
            } if rte == "/" => route_index(stream),
            HTTPRequest {
                verb: HTTPVerb::GET,
                route: rte,
                ..
            } => route_general_path(rte, stream),
            _ => route_error("404", stream),
        }
    }

    fn route_index(mut stream: TcpStream) {
        if let Ok(index) = fs::read_to_string(format!("{}/html/index.html", crate::SRC_ROOT)) {
            let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", index);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            route_error("400", stream);
        }
    }

    fn route_general_path(mut route: String, mut stream: TcpStream) {
        if route.ends_with(".html") {
            route = format!("/html{}", route);
        }

        if let Ok(contents) = fs::read_to_string(format!("{}{}", crate::SRC_ROOT, route)) {
            let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", contents);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            route_error("400", stream);
        }
    }

    fn route_error(errcode: &str, mut stream: TcpStream) {
        let response = format!("HTTP/1.1 {} UnknownResource \r\n\r\n", errcode);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

mod http {

    pub enum HTTPVerb {
        // This is incomplete, I only remember 3 of them
        GET,
        POST,
        DELETE,
        UNKNOWN,
    }

    pub struct HTTPRequest {
        pub protocol: String,
        pub verb: HTTPVerb,
        pub route: String,
        // who cares about headers
    }

    pub fn process_request(request: String) -> HTTPRequest {
        let mut parsed_request = HTTPRequest {
            protocol: "".to_string(),
            verb: HTTPVerb::UNKNOWN,
            route: "".to_string(),
        };

        let lines: Vec<&str> = request.split("\r\n").collect();

        let fst = lines[0].split(' ').collect::<Vec<&str>>();
        if let [method, route, protocol] = &fst[..] {
            parsed_request.route = route.to_string();
            parsed_request.protocol = protocol.to_string();
            parsed_request.verb = match method {
                &"GET" => HTTPVerb::GET,
                &"POST" => HTTPVerb::POST,
                &"DELETE" => HTTPVerb::DELETE,
                _req => HTTPVerb::UNKNOWN,
            }
        }
        parsed_request
    }
}
