use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //creating a simple webserver
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Hello, world!");

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("connection established ! ");

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //creating buffer
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    //creating a response
    let response = "HTTP/1.1 200 OK \r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
