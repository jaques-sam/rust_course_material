fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_does_not_build_due_to_missing_lifetime_annotations() {
        fn longest(x: &str, y: &str) -> &str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let p = 42;
        {
            let q = 10;
            let r = longest(&p, &q);
            println!("Minimum is {}", r);
        }
    }
}
