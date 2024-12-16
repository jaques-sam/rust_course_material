// An integer division that doesn't `panic!`
// Extra box to prove `ref` keyword below
fn checked_division(dividend: i32, divisor: i32) -> Option<Box<i32>> {
    if divisor == 0 {
        None
    } else {
        Some(Box::new(dividend / divisor))
    }
}

fn main() {
    let d = checked_division(0, 1);

    if let Some(ref number) = d {
        println!("Number is {number}");
    }

    println!("Number is {}", d.expect("not null"));
}
