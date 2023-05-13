fn monkey_count(n: i32) -> Vec<i32> {
    (1..=n).collect::<Vec<_>>()
}

fn main() {
    println!("{:?}", monkey_count(10));
}
