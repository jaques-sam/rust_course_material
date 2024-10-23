fn main() {
    let x = -1;

    #[allow(unreachable_patterns)]
    match x {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=9 => println!("within range"),
        x if x > 10 => println!("more than ten: {}", x),
        x => println!("{}", x),  // OR next line
        _ => println!("default Case")  // Match expressions are exhaustive
    }
}
