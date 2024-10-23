
trait Animal {
    fn noise(&self) -> &str { "" } // default impl
}

#[allow(unused)]
struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn noise(&self) -> &str {
        "woof!"
    }
}

impl Dog {  // equal to new<T: Dog>
    fn new(name: &'static str) -> Dog {
        Dog { name }
    }
}

//-----------

struct Zoo {
    animals: Vec<Box<dyn Animal>>,  // Box has a fixed size
}

impl Zoo {
    fn make_some_noise(&self) {
        for animal in self.animals.iter() {
            println!("{}", animal.noise());
        }
    }
}

fn main() {
    let zoo = Zoo {
        animals: vec![Box::new(Dog::new("Fifi"))],
    };

    zoo.make_some_noise();
}
