//have to make function that will handle the routes
pub struct Route_Data {
    route: String,
    function: fn(),
    method: String,
}

//impl for route_data
impl Route_Data {
    // create and returns the instance of the routedata struct
    pub fn new(route: String, function: fn(), method: String) -> Self {
        Self {
            route: route,
            function: function,
            method: method,
        }
    }
}

// //make the function that reutrn individual struct instances
// fn get_struct(route: string, function: fn(), httpverb: string) -> Vec<Route_Data> {
//     //take the elements create a struct instance and then vector array
//     let route_data = Route_Data { route, function };
// }
//
// // function will return the route struct instances
// pub fn route_creator() -> Vec<Route_Struct> {
//     //
// }
