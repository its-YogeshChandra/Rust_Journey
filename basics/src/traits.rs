//traits

pub trait Voice {
    fn speak(value: String) -> Self;
}

pub struct Dog {
    sound: String,
}

impl Voice for Dog {
    fn speak(value: String) -> Self {
        Self { sound: value }
    }
}

#[derive(Debug)]
pub struct Cat {
    sound: String,
}

impl Voice for Cat {
    fn speak(value: String) -> Self {
        Self { sound: value }
    }
}

pub fn playsound() {
    let value = "marco".to_string();
    let cat = Cat::speak(value);
    println!("the cat voice is : {:?}", cat)
}

trait Greeter {
    fn greet(&self);
}
struct Person {
    name: String,
}

impl Greeter for Person {
    fn greet(&self) {
        println! {"hello, {} !", self.name};
    }
}
pub fn greet_person() {
    //figure the issue
    let person = Person {
        name: "marco".to_string(),
    };

    person.greet();
}
