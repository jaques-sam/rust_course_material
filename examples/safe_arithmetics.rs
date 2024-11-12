fn square_checked(num: i32) -> Option<i32> {
    num.checked_mul(num)
}

fn square(num: i32) -> i64 {
    num as i64 * num as i64
}

fn main() {
    println!("Square with overflow check: {:?}", square_checked(46341));
    println!("Square simply using i64: {:?}", square(46341));
}
