fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x < y {
        x
    } else {
        y
    }
}

fn main() {
    let p = "tiny";
    {
        let q = "large";
        let r = longest(p, q);
        println!("Longest is {}", r);
    }
}
