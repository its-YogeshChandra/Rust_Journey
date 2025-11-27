use crate::controller::send_data;
use crate::utils::Request;

//
// pub fn routes_creator(request: Request) -> Vec<RouteData> {
//     // call the routes_data struct with instance function
//     let httpverb = String::from("POST");
//     let path = String::from("/Random");
//     let function_string = String::from("send_data");
//
//     let responseroute = RouteData::new(path, send_data(request), httpverb);
//     //return the vector out of it
//     let resultantpaths = vec![responseroute];
//     resultantpaths
// }

pub fn routes_moderator(request: Request) -> Result<(), String> {
    //check the path in the request object and  then add respective function to it;
    let path = &request.route;

    //match the route and call differnt function
    match path.as_str() {
        "/create" => {
            send_data(request)?;
            Ok(())
        }
        _ => Err("no route exist".to_string()),
    }
}
