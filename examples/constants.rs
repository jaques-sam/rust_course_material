fn main() {}

#[cfg(test)]
mod tests {

    static LANGUAGE: &str = "Rust ❤️"; // `'static` lifetime
    const THRESHOLD: i32 = 10;

    #[test]
    fn test_does_not_build_as_constants_cannot_be_changed() {
        let n = 16;

        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        let _saturated = n > THRESHOLD;

        THRESHOLD = 5; // Error! Cannot assign
        LANGUAGE = "C++"; // Error! Cannot assign
    }
}
