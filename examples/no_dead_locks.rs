use cooptex::*;

fn main() {
    let a = CoopMutex::new(42);

    retry_loop(|| {
        let first = a.lock()?.unwrap();
        let second = a.lock()?.unwrap(); // will panic

        assert_eq!(*first + *second, 84);
        Ok(())
    });
}
