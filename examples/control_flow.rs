fn is_even(x: i32) -> bool {
    x % 2 == 0 // no semi-colon, no return
}

fn main() {
    // if-else statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("{}", number);

    // if-let statement => shorter match w/ 1 branch
    let num = Some(4);
    if let Some(i) = num {
        // creates a var i = num.1 in case num is 'Some'
        println!("number is: {}", i);
    }

    // for loops
    let mut even = Vec::<_>::new();

    for i in (1..4).rev() {
        if is_even(i) {
            even.push(i);
        }
    }

    println!("{:?}", even);
}
