use crate::utils::{
    Request, Response, ResponseBody, errorhandler, handle_response, json_deserializer,
};
use serde_json::Value;
use std::ffi::OsStr;
use std::process::Stdio;
use std::{collections::HashMap, net::TcpStream, process::Command};
pub fn extractor(request: Request, stream: TcpStream) {
    //get the data from the reqeust
    let ResponseBody { data }: ResponseBody<Value> = json_deserializer(&request.body_data);

    // make the object out of the data
    let iterable_data = data.as_object();

    let mut main_data: HashMap<String, String> = HashMap::new();

    // check for the keys in the data
    let keys = ["format", "url"];
    if let Some(data) = iterable_data {
        //iterate the value
        for key in keys {
            //check if fetched object has particular key
            if !data.contains_key(key) {
                //throw the error to the frontend
                let error = format!("{}: key is missing", key);
                errorhandler(&stream, &error)
            } else {
                //add the keys and its value to the obj
                match &data[key] {
                    Value::String(s) if s == "1080p" => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    Value::String(s) if s == "720p" => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    Value::String(s) if s == "480p" => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    Value::String(s) if s == "360p" => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    Value::String(s) if s == "240p" => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    Value::String(s) if s == "144p" => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    Value::String(s) if s.starts_with("http") => {
                        main_data.insert(key.to_string(), data[key].to_string());
                    }
                    _ => {
                        let error = String::from("invalid value");
                        errorhandler(&stream, &error);
                    }
                }
            }

            //fetch the data from the yt-dlp
        }
    }

    //call the yt-dlp
    let video_url = match main_data.get("url") {
        Some(url) => url,
        None => {
            let error = String::from("url key missing");
            errorhandler(&stream, &error);
            return;
        }
    };
    let ytdlp_process = Command::new("yt-dlp")
        .arg("--list-formats")
        .arg(video_url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute the process");

    //fix the data to that string
    let output = String::from_utf8_lossy(&ytdlp_process.stdout);

    //fetch the error
    let error = String::from_utf8_lossy(&ytdlp_process.stderr);
}
