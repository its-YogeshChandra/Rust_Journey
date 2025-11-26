mod utils;

use utils::Route_Data;

pub fn routes_creator() -> Result<Vec<Route_Data>, String> {
    // call the routes_data struct with instance function
    let httpverb = String::from("POST");
    let path = String::from("/Random");
    let responseroute = Route_Data::new(path, send_data, httpverb);

    //return the vector out of it
    let resultantpaths = vec![responseroute];
}
