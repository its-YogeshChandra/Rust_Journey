//bringig the reuest sturct  crate ;
use crate::utils::Request;
use crate::utils::Response;
use crate::utils::handle_response;
use std::net::TcpStream;
// bringing  sirealization crate serde
use crate::utils::errorhandler;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Data {
    name: String,
    username: String,
    age: i32,
}

impl Data {
    pub fn new(name: String, username: String, age: i32) -> Self {
        Self {
            name,
            username,
            age,
        }
    }
}

pub fn send_data(request: Request, stream: TcpStream) -> () {
    // match the argument with the defined set
    let user = request.body_data;
    if user == "adam" {
        let adam = Data::new("adam".to_string(), "Levine".to_string(), 21);

        //create the response struct
        //serealie data : as complex data needs to be serealize in the server
        let data = serde_json::to_string(&adam).expect("error while parsing data");
        let message = String::from("successfully send data");
        let response = Response::new_struct(true, message, data);

        handle_response(response, stream)
    } else {
        let error_val = String::from("name doesn't match to the payload");

        //calling the error handler
        errorhandler(stream, &error_val);
    }
}
