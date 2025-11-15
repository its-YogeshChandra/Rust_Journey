pub struct data {
    name: String,
    username: String,
    age: i32,
}

fn send_response(user: str) -> data {
    //create a user name adam out of struct and return that

    //check for the value in stuct
    let value = String::from("adam");
    if (user == "adam") {
        let adam = data {
            name: String::from("adam"),
            username: String::from("levineXX"),
            age: 23,
        };

        //return the created struct element
        adam
    }
}
