use std::ops::Deref;

struct MyBox<T>(T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);

    let name = Box::new(String::from("Sam"));
    hello(&name); // &Box<String> -> &String -> &str

    hello(&(*name)[..]); // without Deref coercion
}
