use crate::main;
use crate::services::{code_handler, codeextractor};
use crate::utils::{
    Request, Response, ResponseBody, errorhandler, handle_response, json_deserializer,
};
use core::error;
use serde_json::Value;
use std::process::Stdio;
use std::{collections::HashMap, net::TcpStream, process::Command};

#[tokio::main]
pub async fn extractor(request: Request, stream: TcpStream) {
    //get the data from the reqeust
    let ResponseBody { data }: ResponseBody<Value> = json_deserializer(&request.body_data);

    // make the object out of the data
    let iterable_data = data.as_object();

    let mut main_data: HashMap<String, String> = HashMap::new();

    // check for the keys in the data
    let keys = ["bitrate", "url", "content-length", "vcodec"];
    let vcodec: Vec<&str> = vec!["avc1.64002a", "av01.0.09M.08"];
    if let Some(data) = iterable_data {
        //iterate the value
        for key in keys {
            //check if fetched object has particular key
            if !data.contains_key(key) {
                //throw the error to the frontend
                let error = format!("{}: key is missing", key,);
                errorhandler(&stream, &error)
            } else {
                match (key, &data[key]) {
                    //add the bitrate
                    ("bitrate", Value::String(s))
                        if matches!(
                            s.as_str(),
                            "1080p" | "720p" | "480p" | "360p" | "240p" | "144p"
                        ) =>
                    {
                        main_data.insert(key.to_string(), s.clone());
                    }

                    // add the url
                    ("url", Value::String(s)) if s.starts_with("http") => {
                        main_data.insert(key.to_string(), s.clone());
                    }

                    //add the content-length
                    ("content-length", Value::String(s)) => {
                        if s.parse::<u64>().is_ok() {
                            main_data.insert(key.to_string(), s.clone());
                        } else {
                            let error = "content length must be a number";
                            errorhandler(&stream, error);
                        }
                    }

                    //add the video codec
                    ("vcodec", Value::String(s)) if vcodec.contains(&s.as_str()) => {
                        main_data.insert(key.to_string(), s.clone());
                    }

                    ("bitrate", _) => {
                        errorhandler(&stream, "invalid bitrate value");
                        return;
                    }

                    ("url", _) => {
                        errorhandler(&stream, "invalid url value");
                        return;
                    }
                    ("content-length", _) => {
                        errorhandler(&stream, "invalid content-length value");
                    }

                    ("vcodec", _) => {
                        errorhandler(&stream, "invalid vcodec value");
                        return;
                    }
                    (_, _) => errorhandler(&stream, "invalid payload"),
                }
            }
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

    //call the ytdlp_process to fetch the data
    let ytdlp_process = Command::new("yt-dlp")
        .arg("--list-formats")
        .arg(video_url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute the process");

    //fix the data to that string
    let output = String::from_utf8_lossy(&ytdlp_process.stdout);

    //get the error
    let error = String::from_utf8_lossy(&ytdlp_process.stderr);

    //fetch the important code according to the
    if !output.is_empty() {
        //call the codeextractor
        //the bitrate
        let bitrate = match main_data.get("bitrate") {
            Some(bitrate) => bitrate,
            None => {
                let error = String::from("birate key is missing");
                errorhandler(&stream, &error);
                return;
            }
        };

        //the vidoe codec
        let vcodec = match main_data.get("vcodec") {
            Some(vcodec) => vcodec,
            None => {
                let error = String::from("vcodec key is missing");
                errorhandler(&stream, &error);
                return;
            }
        };

        //destructure the tuple
        let (vidcode, audcode) = code_handler(&output.to_string(), bitrate, vcodec);

        //fix the issue :
        println!("vid codes: {:?}", vidcode);
        println!("audio codes: {:?}", audcode);

        //download the video using the codes
        let downlaoder_ytdlp = Command::new("yt-dlp")
            .arg("--list-formats")
            .arg(video_url)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("failed to execute the process");
    }
}
