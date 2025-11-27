pub struct Data {
    name: String,
    username: String,
    age: i32,
}

pub fn send_data(user: String) -> Result<Data, ()> {
    //create a user name adam out of struct and return that

    //check for the value in stuct
    let value = String::from("adam");
    if user == "adam" {
        let adam = Data {
            name: String::from("adam"),
            username: String::from("levineXX"),
            age: 23,
        };

        //return the created struct element
        adam
    } else {
        println!("error string doesn't match");
    }
}
