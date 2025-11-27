use crate::controller::send_data;
use crate::utils::routehandler::Routes_Data;

pub fn routes_creator() -> Result<Vec<Routes_Data>, String> {
    // call the routes_data struct with instance function
    let httpverb = String::from("POST");
    let path = String::from("/Random");
    let responseroute = Routes_Data::new(path, send_data, httpverb);

    //return the vector out of it
    let resultantpaths = vec![responseroute];
    resultantpaths
}
