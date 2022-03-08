use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let tcplistener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in tcplistener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_connection(stream);
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
