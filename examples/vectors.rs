#[allow(clippy::useless_vec)]
fn main() {
    let mut v1: Vec<i32> = Vec::new();  // optional type forcing
    let v2 = vec![1, 2, 3];  // type i32 inferred

    v1.push(1);
    let _third: &i32 = &v2[2]; // can panic, use .get() instead
    v1.iter_mut().for_each(|e| *e += 1);
    let _v3: Vec<i32> = v1.iter().map(|e| e + 1).collect();
    // Did you notice something different between last 2 lines? In using `e`?
}
