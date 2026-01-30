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

pub struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn announce(&self) {
        println!("the print is : {},", self.part);
    }
}

pub fn figure_value() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = Excerpt {
        part: first_sentence,
    };
    i.announce();
}

pub struct Container<'a> {
    inner: &'a str,
}

pub fn illegal_move() -> Container<'static> {
    let val = String::from("I am temporary");
    let c = Container { inner: &val };
    c
}
