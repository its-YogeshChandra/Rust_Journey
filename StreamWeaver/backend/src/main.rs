use std::fs::File;
use std::io::Read;
use std::io::Write;
// use std::io::prelude::*;
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

fn handle_connection(mut stream: TcpStream) -> String {
    //creating buffer
    //Buffers in Rust: In Rust, a buffer is typically a block of memory used for temporary storage of data. Buffers are commonly used when reading or writing data to or from sources like files, network sockets, or memory
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        println!("inside the function");
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(n) => {
                println!("reading done : {}", n);
            }
            Err(e) => {
                eprintln!("failed to read from file: {}", e);
            }
        }
        //creating a response
        let response = format!(
            "HTTP/1.1 200 OK \r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        let working = String::from("working fine");
        working
    } else {
        println!("invalid route or request type");
        let error = String::from("invalid route or request type");
        error
    }
}

//reeuest format
//HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
