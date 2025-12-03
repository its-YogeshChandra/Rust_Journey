//controller for getting meta data
//bringig the reuest sturct  crate ;
use crate::utils::Request;
use crate::utils::Response;
use crate::utils::handle_response;
use std::net::TcpStream;
// bringing  sirealization crate serde
use crate::utils::errorhandler;
use crate::utils::{ResponseBody, json_deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::map::Keys;

//#[derive(Serialize, Deserialize)]
enum MetaInformation {
    Url(Value),
}

pub fn meta_data_and_options(request: Request, stream: TcpStream) -> () {
    // get the data from the request
    let ResponseBody { data }: ResponseBody<Value> = json_deserializer(&request.body_data);

    //match the data for;
    let iterable_data = data.as_object();

    //create a map of serde value to put the dat into
    let obj = serde_json::Map::new();
    let mut body_data = Value::Object(obj);

    //match for the key
    let keys = ["url"];
    if let Some(data) = iterable_data {
        //iterate the keys array
        for key in keys {
            //match the key with the obj
            if !data.contains_key(key) {
                //throw the error
                let error = format!("{} not found ", key);
                errorhandler(&stream, &error)
            } else {
                body_data[key] = data[key].clone()
            }
        }
    }

    //call the youtube api for the metadata available
}
