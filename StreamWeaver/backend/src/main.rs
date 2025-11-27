use std::io::Read;
// use std::io::prelude::*;
use std::fs::File;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
//import the route creator from routes folder
mod routes;
//import utils module
mod utils;
//import controller module
mod controller;

use routes::routes_creator;
use utils::Request;

fn main() {
    //checking the routes
    let route_data = routes_creator();

    //creating a simple webserver
    let listener = TcpListener::bind("127.0.0.1:8080");

    // error handling if error while handling error
    match listener {
        Ok(connection) => {
            println! {"connection successfully established"};

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

//option using impl

fn handle_connection(mut stream: TcpStream) {
    //creating buffer
    //Buffers in Rust: In Rust, a buffer is typically a block of memory used for temporary storage of data. Buffers are commonly used when reading or writing data to or from sources like files, network sockets, or memory
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("raw stream data : {:?}", &stream);

    //read the buffer put and seperate different parts out of it

    let request = String::from_utf8_lossy(&buffer[..]);

    //print the request
    println!("Request : {}", request);

    //make instance of request and update the values
    let mut request_data = Request::new(
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
        "random".to_string(),
    );

    //extract the values from the request
    for part in request.split("\r\n") {
        println!("------printing parts ------");
        println! {"{}", part};
        println!("------------");

        //match the data and update the request_data object
        if part.starts_with("GET")
            || part.starts_with("POST")
            || part.starts_with("DELETE")
            || part.starts_with("PUT")
        {
            for items in part.split(" ") {
                match items {
                    "GET" | "POST" | "PUT" | "DELETE" => request_data.method = items.to_string(),
                    s if s.starts_with("HTTP") => request_data.httpversion = s.to_string(),
                    "/" => request_data.route = items.to_string(),
                    _ => {
                        eprintln!("the data is empty");
                    }
                }
            }
        }
        // for content type
        else if part.starts_with("Content-Type") {
            request_data.content_type = part.to_string()
        }
        // for host
        else if part.starts_with("Host") {
            for items in part.split(" ") {
                println!("items: {}", items);
                match items {
                    s if s.starts_with("1") => request_data.host = s.to_string(),
                    _ => {
                        println!(" ");
                    }
                }
            }
        }
        //for body data if present
        else if part.starts_with("{") {
            request_data.body_data = part.to_string()
        }
    }
    println!("http_version : {}", &request_data.httpversion);
    println!("body_data : {}", &request_data.body_data);
    println!("host : {}", &request_data.host);

    //read from the file
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

    //sending data through stream
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    //sending of the maker

    // } else {
    //     println!("invalid route or request type");
    //     let error = String::from("invalid route or request type");
    //     error
}

//request format
//HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
