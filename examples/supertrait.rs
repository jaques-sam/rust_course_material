trait Shape {
    fn area(&self) -> f64;
}

trait Circle : Shape {  // or `Circle where Self: Shape`
    fn radius(&self) -> f64 {
        (self.area() / std::f64::consts::PI).sqrt()
    }
}

fn print_area_and_radius<C: Circle>(c: C) {
    // Here we call the area method from the supertrait `Shape` of `Circle`.
    println!("Area: {}", c.area());
    println!("Radius: {}", c.radius());
}

struct Euro {}

impl Shape for Euro {
    fn area(&self) -> f64 {
        25.0
    }
}

impl Circle for Euro {
}

fn main() {
    let one_euro = Euro{};
    print_area_and_radius(one_euro);
}
