fn increment(n: &mut u32) {
    /* dereferencing is always needed for &mut,
       Rust prioritizes safety and control when modifying data.
       Requiring explicit dereferencing with mutable references */
    *n += 1;

}

fn main() {
    let mut x = 1;
    increment(&mut x);
    println!("{}", x);
}
