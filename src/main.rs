use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

struct Request {
    request_type: String,
    request_page: String,
    protocol: String,
}

impl Request {
    fn get_request_type(&self) -> &str {
        &self.request_type
    }

    fn get_request_page(&self) -> &str {
        &self.request_page
    }

    fn get_protocol(&self) -> &str {
        &self.protocol
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request = split_request(&String::from_utf8_lossy(&buffer[..]));

    // let request_file: String = {
    //     match request.get_request_page() {
    //         "/" => format!("./public{}index.html", request.get_request_page()),
    //         _ => format!("./public{}", request.get_request_page()),
    //     }
    // };

    let mut request_file: String = format!("./public{}", request.get_request_page());

    let request_type = request.get_request_type();
    let request_protocol = request.get_protocol();

    if Path::new(&request_file).is_dir() {
        request_file = format!("{}/index.html", request_file);
    }

    let content = match fs::read_to_string(&request_file) {
        Ok(file) => file,
        Err(_) => {
            println!("cannot find file: {}", request_file);
            String::from("Cannot find file")
        }
    };

    let content_type = Path::new(&request_file).extension().unwrap_or_default();

    let content_type = content_type_creator(content_type.to_str().unwrap_or_default());

    println!("\n===================================================================");
    println!("type: {}", request_type);
    println!("page: {}", request.get_request_page());
    println!("protocol: {}", request_protocol);
    println!("directory to get file(not from request): {}", request_file);
    println!("content type  {}", content_type);
    println!("===================================================================\n");

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-type: {}\r\n\r\n{}",
        content_type, content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn split_request(request: &str) -> Request {
    let mut iter = request
        .split('\n')
        .flat_map(|line| line.split(' '))
        .filter(|word| word.len() > 0);

    let request_type = iter.next().unwrap_or_default().to_owned();
    let request_page = iter.next().unwrap_or_default().to_owned();
    let protocol = iter.next().unwrap_or_default().to_owned();

    Request {
        request_type: request_type,
        request_page: request_page,
        protocol: protocol,
    }
}

fn content_type_creator(file_extension: &str) -> &str {
    match file_extension {
        "js" => "application/javascript",
        "html" => "text/html",
        "css" => "text/css",
        "gif" => "image/gif",
        "png" => "image/png",
        "jpeg" => "image/jpeg",
        "tiff" => "image/tiff",
        "svg" => "image/svg+xml",
        "mpeg" => "video/mpeg",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "xml" => "text/xml",
        _ => "",
    }
}
