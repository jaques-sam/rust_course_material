fn is_even(x: i32) -> bool
{
    x % 2 == 0
}

fn main() {
    let even: Vec<_> = [0, 1, 2, 3, 4].iter().filter(|&x| is_even(*x)).collect();

    println!("{:?}", even);
}
