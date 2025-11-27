//bringig the reuest sturct  crate ;
use crate::utils::Request;

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

pub fn send_data(request: Request) -> Result<Data, String> {
    // match the argument with the defined set
    let user = request.body_data;
    if user == "adam" {
        let adam = Data::new("adam".to_string(), "Levine".to_string(), 21);
        Ok(adam)
    } else {
        let error_val = String::from("wrong data");
        Err(error_val)
    }
}
