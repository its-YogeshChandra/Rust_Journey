//for box smartpointer
pub fn smartpointer() {
    let small = Box::new(10);
    let heap_value = small;

    let heap = *heap_value + 5;
    println!("the heap value is :{}", heap);
}

//for recursive data type
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn recursive_data() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("the list value is : {:?}", list);
}

