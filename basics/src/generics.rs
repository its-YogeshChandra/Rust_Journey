//how to do the generic part
#[derive(Debug)]
pub struct Point<T, Y> {
    x: T,
    y: Y,
}

impl<T, Y> Point<T, Y> {
    fn mixup<T1, Y1>(self, other: Point<T1, Y1>) -> Point<T, Y1> {
        //return the struct
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn createpoint() {
    //call the function in this
    let p1 = Point { x: 2, y: "fixer" };

    let p2 = Point { x: 4, y: "marco" };

    let newpoint = Point::mixup(p1, p2);
    println!("the value is : {:?}", newpoint);

    let a = 12;
    let b = 54;

    let largervalue = get_larger(a, b);
    println!("the larger value is : {:?}", largervalue);
}
//trait bound thing
pub fn get_larger<T: PartialOrd>(a: T, b: T) -> T {
    //get the value
    if a > b {
        return a;
    } else {
        return b;
    }
}

//simplified version of rust option enum
pub enum OptionValue<T> {
    Some(T),
    None,
}

pub fn optionhandler() {
    let mut values: Vec<i32> = Vec::new();
    //create my option value
    let x = OptionValue::Some(5);
    let y: OptionValue<i32> = OptionValue::None;

    match x {
        OptionValue::Some(v) => {
            println!("value is {}", v);
            values.push(v);
        }
        OptionValue::None => println!("there is no value"),
    }
    println!("the vector is : {:?}", values);
}

//generics do the monomorphizaton converting gneric codes into specific codes,

//function for explanation
//`rustc --explain E0369`
