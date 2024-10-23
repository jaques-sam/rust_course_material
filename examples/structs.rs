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

    fn create_from_template() -> User {
        User {
            username: "unknown".to_string(),
            active: true,
        }
    }
}

#[derive(Debug)]
struct ColorRGB(i32, i32, i32);  // Tuple struct

fn main() {
    let sj = User {  // Instantiation
        username: String::from("samja"),
        active: true,
    };
    let bk = User {
        username: String::from("BK"),
        ..User::create_from_template()
    };
    let black = ColorRGB(0, 0, 0);
    let user_active = sj.is_active();

    println!("{:?}", sj);  // using Debug
    println!("{:?}", black);  // using Debug
}
