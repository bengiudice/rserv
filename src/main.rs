use std::{
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
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap(); //TODO: handle error
}
