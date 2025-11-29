// create function that returns the parsed (deserialized data )

//import serde json
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]

pub struct ResponseBody<T> {
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn create(data: T) -> Self {
        Self { data }
    }
}

// function to desirealize the data
pub fn json_deserializer<'de, T>(datastring: &'de str) -> ResponseBody<T>
where
    T: Deserialize<'de>,
{
    // take the input and make the function
    let json_data = serde_json::from_str(&datastring)
        .expect("error while deseralizing data in json_deserializer");

    // create the function
    let return_val = ResponseBody::create(json_data);
    return_val
}
