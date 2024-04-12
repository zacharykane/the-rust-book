use std::ops::Deref;

// just a Tuple Struct that can hold anything
pub struct MyBox<T: std::fmt::Debug>(T);

// static method to create an instance of MyBox<T>
impl<T: std::fmt::Debug> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// you need to implement the Deref Trait to be able to use the deref operator *
impl<T: std::fmt::Debug> Deref for MyBox<T> {
    type Target = T;

    // in our custom case, return the value inside the tuple
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox! {:?}", self.0);
    }
}

pub fn execute() {
    let x = 5;
    let a = MyBox::new(x);
    assert_eq!(5, *a);

    let message = MyBox::new(String::from("Rust"));
    // will auto de-reference our MyBox into a &str slice
    hello(&message);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
