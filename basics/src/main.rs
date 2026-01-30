mod generics;
mod lifetimes;
mod traits;

use generics::createpoint;
use lifetimes::{figure_value, illegal_move, liftime_function, static_main};
use traits::{greet_person, playsound};

use crate::generics::optionhandler;
fn main() {
    createpoint();
    optionhandler();

    //tratis
    println!("traits");
    greet_person();
    playsound();

    //lifetimes
    liftime_function();
    figure_value();
    static_main();
    let value = illegal_move();
    println!("the container is : {:?}", value);
}
