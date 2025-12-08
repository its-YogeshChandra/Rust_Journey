use crate::utils::errorhandler;
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

pub fn format_handler(
    output: &str,
    stream: TcpStream,
) -> Result<HashMap<String, HashMap<String, String>>, ()> {
    let mut format_data: HashMap<String, HashMap<String, String>> = HashMap::new();
    //fetch the value
    for lines in output.lines() {
        //match  lines against presence of different resolution
        match lines {
            s if s.contains("mp4") => {
                for parts in s.lines() {
                    //fixing the error
                    match parts {
                        m if m.contains("1920x1080") && m.contains("vp9") => {}
                        n if n.contains("1280x720") && n.contains("vp9") => {}
                        _ => {
                            let error = String::from("error while reading output");
                            errorhandler(&stream, &error)
                        }
                    }
                }
            }
            n if n.contains("webm ") && n.contains("video") => {}
            p if p.contains("webm") && p.contains("audio") => {}
            m if m.contains("m4a") => {}
            _ => {
                let error = String::from("error while reading output");
                errorhandler(&stream, &error)
            }
        }
    }
}
