use std::fs::File;
use std::io::Read;
use std::io::Write;
// use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //creating a simple webserver
    let listener = TcpListener::bind("127.0.0.1:8080");
    println!("Hello, world!");

    // error handling if error while handling error
    match listener {
        Ok(connection) => {
            println! {"connection successfully established"};
            // println! {"connection : {:?}", &connection};

            // loop and check the stream from listener
            for stream in connection.incoming() {
                //error handling for stream
                match stream {
                    Ok(streamdata) => {
                        println! {
                        " data stream is receiving : {:?}", streamdata};

                        //connect to the handling stream
                        handle_connection(streamdata);
                    }
                    Err(error) => {
                        eprint!("error while getting stream ");
                        eprint!("error: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println! {"error while establishing connection "};
            println! {"error: {}", error};
        }
    }
}

//create request struct

struct Request {
    httpversion: String,
    host: String,
    route: String,
    method: String,
    data: String,
}

enum request_element {
    HTTP,
    GET,
    POST,
    PUT,
    DELETE,
}

fn handle_connection(mut stream: TcpStream) -> String {
    //creating buffer
    //Buffers in Rust: In Rust, a buffer is typically a block of memory used for temporary storage of data. Buffers are commonly used when reading or writing data to or from sources like files, network sockets, or memory
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    //read the buffer put and seperate different parts out of it

    let request = String::from_utf8_lossy(&buffer[..]);

    //print the request
    println!("Request : {}", request);

    //make instance of request and update the values
    let mut request_data = Request {
        data: String::from("random"),
        httpversion: String::from("random"),
        host: String::from("random"),
        method: String::from("random"),
        route: String::from("random"),
    };

    //extract the values from the request
    for part in request.split(" ") {
        println!("------printing parts ------");
        println! {"{}", part};
        println!("------------");

        //match the data and update the request_data object
        match part {
            request_element::HTTP => request_data.httpversion = String::from(&part),
        }
    }

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
