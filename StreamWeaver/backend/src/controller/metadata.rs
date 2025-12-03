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
}
