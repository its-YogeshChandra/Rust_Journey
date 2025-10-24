use std::io;

fn main() {
// what can be done is calling other functions in main

//welcome message
    println!("=====================================
 Welcome to the CLI Calculator! 
=====================================
Perform basic math operations with ease");
    
//instruction
println!("how to use : ");
println!("type number 1 ");
println!("type second number");
println!("select the operation");
println!("addition: add
substraction: sub
multiplication : mult
division: div
remainder: rem");

//start and stop
let mut switch = String::new();
io::stdin().read_line(&mut switch).expect("failed to read line");

//check for the switch what it is 
if switch == "start"{
  let (numone, numtwo, operation) = take_input()

}



}

fn take_input() -> Option<(i32, i32, String)>{
let mut _num1 = String::new();
let mut _num2 = String::new();
let mut operation= String::new();
let opparray = vec!["add", "sub", "mult", "div", "rem"];
    
//ask the quesiton and then take the input

println!("add first number");
io::stdin().read_line( &mut _num1).expect("failed to read the number");
   //change the string to number
let num_one: i32 = _num1.parse().expect("not a number");

println!("add second number");
io::stdin().read_line( &mut _num2).expect("failed to read the number");
let num_two: i32 = _num2.parse().expect("not a number");

println!("choose the operation");
io::stdin().read_line( &mut operation).expect("failed to read the number");
//check if the operation is not correct
let operation = String::from(operation);
let opp : &str = &operation;
if opparray.contains(&opp){
Some((num_one, num_two, operation))
}
else{
println!("invalid operation");
None
}

}

fn perform_operation(first: i32, second: i32, operation: String){

}



