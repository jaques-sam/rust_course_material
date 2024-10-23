#[allow(unused)]
enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),  // tuple struct
}  // biggest value size => Message size

impl Message {
    fn call(&self) {
        // tadaaa
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
