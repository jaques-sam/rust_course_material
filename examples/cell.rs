use std::cell::Cell;

fn main() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());
}

#[cfg(test)]
mod tests {
    fn test_does_not_build_due_to_second_mut_borrow() {
        let mut x = 1;
        let y = &mut x;
        let z = &mut x;
        x = 2;
        *y = 3;
        *z = 4;
        println!("{}", x);
    }
}
