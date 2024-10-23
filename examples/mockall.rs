fn main() {}

#[cfg(test)]
mod test {
    use mockall::automock;

    #[automock]
    #[allow(dead_code)]
    trait MyTrait {
        fn foo(&self) -> u32;
        fn bar(&self, x: u32, y: u32) -> u32;
    }

    #[test]
    fn test_mockall() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().return_const(42u32);
        mock.expect_bar().returning(|x, y| x + y);
    }
}
