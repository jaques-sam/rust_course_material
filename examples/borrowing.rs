fn main() {}

#[cfg(test)]
mod tests {
    fn hold_my_vec<T>(_: Vec<T>) {}

    #[test]
    fn does_not_build_because_of_a_borrowing_issue() {
        let v = vec![1, 2, 3, 5, 7, 11, 13, 17];
        hold_my_vec(v); // culprit
        let element = v.get(3);

        println!("Element '{:?}' taken from vector", element);
    }
}
