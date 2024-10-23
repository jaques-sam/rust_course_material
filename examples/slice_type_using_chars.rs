fn first_word(s: &str) -> &str {
    for (i, item) in s.char_indices() {
        if item == ' ' {
            return &s[0..i];
        }
    }

    s // == &s[..]
}

fn main() {
    println!("{}", first_word("Hello world"));
}
