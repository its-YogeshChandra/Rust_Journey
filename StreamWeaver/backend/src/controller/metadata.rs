//controller for getting meta data
//bringig the reuest sturct  crate ;

use crate::utils::Request;
use crate::utils::Response;
use crate::utils::handle_response;
use dotenv::dotenv;
use std::env;
use std::net::TcpStream;
// bringing  sirealization crate serde
use crate::utils::errorhandler;
use crate::utils::{ResponseBody, json_deserializer};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::map::Keys;
use std::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

//#[derive(Serialize, Deserialize)]
enum MetaInformation {
    Url(Value),
}

#[tokio::main]
pub async fn meta_data_and_options(request: Request, stream: TcpStream) -> () {
    dotenv().ok();
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
                // add the data into the data value obj
                body_data[key] = data[key].clone()
            }
        }
    }

    //call the youtube api for the metadata available
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");

    //set the channel ID
    // let url = format!(
    //     "http://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
    //     api_key,
    //
    // );

    //call the yt-dlp
    let child = Command::new("yt-dlp")
        .arg("--list-formats")
        .arg(url)
        .output()
        .expect("failed to execute the process");
    //
    //
}
