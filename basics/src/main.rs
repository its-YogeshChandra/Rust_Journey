mod generics;
mod lifetimes;
mod traits;

use generics::createpoint;
use lifetimes::{figure_value, liftime_function};
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
}
