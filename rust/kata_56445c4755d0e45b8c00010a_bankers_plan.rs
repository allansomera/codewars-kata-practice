fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    println!("f0: {}\np: {}\nc0: {}\nn: {}\ni: {}", f0, p, c0, n, i);
    true
}

fn main() {
    println!("{:?}", fortune(100_000, 1.0, 2_000, 15, 1.0));
}
