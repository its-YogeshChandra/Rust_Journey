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

//the fix :
#[derive(Debug)]
pub struct Container {
    inner: String,
}
pub fn illegal_move() -> Container {
    let val = String::from("I am temporary");
    let c = Container { inner: val };
    c
}

// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn static_main() {
    {
        // Make a `string` literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
    }

    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
