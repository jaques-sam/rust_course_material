fn main() {}

#[cfg(test)]
mod tests {

    static LANGUAGE: &str = "Rust ❤️"; // `'static` lifetime
    const THRESHOLD: i32 = 10;

    #[test]
    fn test_does_not_build_as_constants_cannot_be_changed() {
        const N: i32 = 16i32; // optional type suffix

        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        let _saturated = N > THRESHOLD;

        THRESHOLD = 5; // Error! Cannot modify a `const`.
        LANGUAGE = "C++"; // Error!
    }
}
