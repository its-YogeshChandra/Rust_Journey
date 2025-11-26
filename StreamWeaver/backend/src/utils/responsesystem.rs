// response struct and its functions

pub struct Response {
    success: bool,
    message: String,
    data: String,
}

impl Response {
    pub fn new_struct(sucess: bool, message: String, data: String) -> Self {
        Self {
            success: success,
            message: message,
            data: data,
        }
    }
}
