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
        //match  lines against presence of different resoltion
        match lines {
            s if s.contains("mp4") => {}
            p if p.contains("webm") => {}
            m if m

            _ => {
                let error = String::from("error while reading output");
                errorhandler(&stream, &error)
            }
        }
    }
}
