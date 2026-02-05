use std::ops::Deref;

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

//making custom smartpointer
struct MainBox<T>(T);

impl<T> MainBox<T> {
    fn new(x: T) -> MainBox<T> {
        MainBox(x)
    }
}

//implement the deref trait
impl<T> Deref for MainBox<T> {
    type Target = T;

    //implement the deref
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn regular_reference() {
    let x = 5;
    let y = MainBox::new(x);

    //function is working fine
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
