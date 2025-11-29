use std::io::Write;
use std::net::TcpStream;

//error response handler
pub fn errorhandler(mut stream: &TcpStream, error: &str) {
    let error = error.to_string();

    //creating a error response
    let response = format!("HTTP/1.1 400 Request Error\r\n{}", error);

    // sending data through stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
