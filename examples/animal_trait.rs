trait Animal {
    fn noise(&self) -> &str {
        "" // default impl
    }
    fn has_tail(&self) -> bool; // expect implementation in struct
}

struct Dog {
    name: &'static str,
} // lifetime annotation mandatory

impl Animal for Dog {
    fn noise(&self) -> &str {
        "woof!"
    }
    fn has_tail(&self) -> bool {
        true
    }
}

impl Dog {
    fn new(name: &'static str) -> Self {
        // equal to new<T: Dog>
        Dog { name }
    }
    fn fetch(&self) {}
}

fn main() {
    let dog = Dog::new("Fifi");
    println!("Dog {} does {}", dog.name, dog.noise());

    dog.fetch();
    assert!(dog.has_tail());
}
