use std::io::Read;
// use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
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

//create request struct

struct Request {
    httpversion: String,
    host: i32,
    route: String,
    method: String,
    body_data: String,
    content_type: String,
    params_data: String,
}

// enum RequestElement {
//     HTTP(String),
//     GET(String),
//     POST(String),
//     PUT(String),
//     DELETE(String),
// }

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
    let mut request_data = Request {
        body_data: String::from("random"),
        httpversion: String::from("random"),
        host: 1234,
        method: String::from("random"),
        route: String::from("random"),
        content_type: String::from("random"),
        params_data: String::from("random "),
    };
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
                    s if s.starts_with("1") => request_data.host = s.parse::<i32>().unwrap(),
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
    // if buffer.starts_with(get) {
    //     println!("inside the function");
    //
    //     let mut file = File::open("hello.html").unwrap();
    //
    //     let mut contents = String::new();
    //     match file.read_to_string(&mut contents) {
    //         Ok(n) => {
    //             println!("reading done : {}", n);
    //         }
    //         Err(e) => {
    //             eprintln!("failed to read from file: {}", e);
    //         }
    //     }
    //
    //     //creating a response
    //     let response = format!(
    //         "HTTP/1.1 200 OK \r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );
    //
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    //
    //     let working = String::from("working fine");
    //     working
    // } else {
    //     println!("invalid route or request type");
    //     let error = String::from("invalid route or request type");
    //     error
}

//request format
//HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
