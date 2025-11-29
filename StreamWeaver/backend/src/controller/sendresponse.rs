//bringig the reuest sturct  crate ;
use crate::utils::Request;
use crate::utils::Response;
use crate::utils::handle_response;
use std::net::TcpStream;
// bringing  sirealization crate serde
use crate::utils::errorhandler;
use crate::utils::{ResponseBody, json_deserializer};
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
#[derive(Serialize, Deserialize)]

// response data that has to be send
pub struct Data {
    name: String,
    username: String,
    age: i32,
}

// impl data for Data block
impl Data {
    pub fn new(name: String, username: String, age: i32) -> Self {
        Self {
            name,
            username,
            age,
        }
    }
}

//create the struct
enum ResType {
    Name(String),
}

// controller function that send_data when called with certain data;
pub fn send_data(request: Request, stream: TcpStream) -> () {
    // match the argument with the defined set
    let user = json_deserializer(&request.body_data);

    //take the data out of it
    let body: Value = user.data;
    // create a match system that match for the particular keys in the gained struct data;
    if let Some(name) = user.data.get("name").and_then(|v| v.as_str()) {
        if name == "adam" {
            println!("the name is adam");
        }
    }
    // check that if data field match to this name ;
    if bodydata {
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
