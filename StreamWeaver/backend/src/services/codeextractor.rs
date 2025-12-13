//extract the bitrate codes

pub fn code_handler(output: &str, bitrate: &str) -> String {
    println!("printing output : {}", output);

    let mut mainstring: String = String::new();
    //loop the output line by line
    for line in output.lines() {
        //check for different formats
        if line.contains(bitrate) && line.contains("mp4") {
            //split the line and extact the code out of it
            let parts: Vec<&str> = line.split_whitespace().collect();
            let code = parts.get(0).clone().expect("not a string value");
            mainstring = code.to_string();
        }
    }
    mainstring
}
