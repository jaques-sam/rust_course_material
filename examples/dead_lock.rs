use std::sync::Mutex;

fn main() {
    let data = Mutex::new(0);
    let _d1 = data.lock();
    let _d2 = data.lock(); // cannot lock, since _d1 is still active

    println!("is not printed");
}
