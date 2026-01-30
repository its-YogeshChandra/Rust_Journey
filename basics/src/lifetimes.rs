//the main function
pub fn liftime_function() {
    let string1 = String::from("the main marco");
    let string2 = "the main marco";
    let result = longest(&string1, string2);
    println!("the longest string is {result}")
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
