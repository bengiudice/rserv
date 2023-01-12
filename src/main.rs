use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //TODO: handle error.
    for stream in listener.incoming() {
        let stream = stream.unwrap(); //TODO: handle error
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap()) //TODO: handle error.
        .take_while(|line| !line.is_empty())
        .collect();
    dbg!(http_request);
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap(); //TODO: handle error
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); //TODO: handle error
}
