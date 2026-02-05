mod generics;
mod lifetimes;
mod smartpointers;
mod traits;

use crate::generics::optionhandler;
use generics::createpoint;
use lifetimes::{figure_value, illegal_move, liftime_function, static_main};
use smartpointers::{recursive_data, smartpointer};
use traits::{greet_person, playsound};
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

    //smartpointers
    smartpointer();
    recursive_data();
}
