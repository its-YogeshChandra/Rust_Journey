use std::io;
use rand::prelude::*;
use std::cmp::Ordering;
fn main() {
    // mention all the statements
    let welcome = "welcome to the word guessing game";
    let rules1 = "choose a random nubmer from 1 to 50"; 
    
       
    println!("{}",  welcome);
    println!( "{}", rules1);

//create a mutable state
let mut guess = String::new();

//load the input into guess

io::stdin().read_line(&mut guess).expect("failed to fetch input");

//make a mechanism that generate ramdom number between 1 to 50 
let mut rng = rand::rng();

//generate a sequeence and choose one from it
let mut nums: Vec<i32> = (1..50).collect::<Vec<i32>>();
nums.shuffle(&mut rng);

let _ = nums.choose(&mut rng);
let newnumber  = rng ;
let mainnumber = newnumber.to_String();
//compare the rng with the guess and 
match guess.cmp(rng){
Ordering::Less => println!("Value too small"),
Ordering::Greater => println!("Too big"),
Ordering::Equal => println!("you win")
}


}
