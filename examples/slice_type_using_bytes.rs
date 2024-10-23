fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s // == &s[..]
}

fn main() {
    let s = "Hello world".to_owned(); // or .to_string()
    let _hello = &s[0..5];
    let _world = &s[6..11];

    println!("{}", first_word("Hello world"));
}
