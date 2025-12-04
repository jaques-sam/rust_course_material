trait Animal {
    fn noise(&self) -> &str;
}

struct Zoo<T: Animal> {
    animals: Vec<T>,
}

impl<T: Animal> Zoo<T> {
    fn make_some_noise(&self) {
        for animal in self.animals.iter() {
            println!("{}", animal.noise());
        }
    }
}

fn main() {
    struct Dog {
        _name: &'static str,
    }

    impl Animal for Dog {
        fn noise(&self) -> &str {
            "woof!"
        }
    }

    impl Dog {
        fn new(name: &'static str) -> Self {
            // equal to new<T: Dog>
            Dog { _name: name }
        }
    }

    let dog = Dog::new("Fifi");
    let zoo = Zoo {
        animals: vec![dog],
    };

    zoo.make_some_noise();
}
