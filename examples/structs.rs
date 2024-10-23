#![allow(dead_code, unused)]

#[derive(Debug)]
struct User {  // Definition
    username: String,
    active: bool,
}

impl User {  // Implementation
    fn is_active(&self) -> bool {
        self.active
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);  // Tuple struct

fn main() {
    let sj = User {  // Instantiation
        username: String::from("samja"),
        active: true,
    };
    let user_active = sj.is_active();

    let black = Color(0, 0, 0);

    println!("{:?}", sj);  // using Debug
    println!("{:?}", black);  // using Debug
}
