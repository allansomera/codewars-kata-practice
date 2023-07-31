fn thirt(n: i64) -> i64 {
    let repeat = vec![1, 10, 9, 12, 3, 4];
    let num: Vec<i64> = n
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("{:?}", num);
    3
}

fn main() {
    println!("{:?}", thirt(1234567));
}
