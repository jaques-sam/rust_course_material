trait Animal {
    fn noise(&self) -> String {
        String::from("") // default impl
    }
    fn has_tail(&self) -> bool; // TO IMPLEMENT
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn noise(&self) -> String {
        String::from("woof!")
    }
    fn has_tail(&self) -> bool {
        true
    }
}

impl Dog {
    fn new(name: String) -> Self {
        // equal to new<T: Dog>
        Dog { name } // omit param name: param name == arg name
    }
    fn fetch(&self) {}
}

fn main() {
    let dog = Dog::new(String::from("Fifi"));
    println!("Dog {} does {}", dog.name, dog.noise());

    dog.fetch();
    assert!(dog.has_tail());
}
