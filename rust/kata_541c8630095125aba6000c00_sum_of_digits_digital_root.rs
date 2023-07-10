fn digital_root(n: i64) -> i64 {
    let v: Vec<i64> = n
        .to_string()
        .chars()
        .map(|x| x.to_string())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("{:?}", v);
    v.into_iter().sum()
}

fn main() {
    println!("{:?}", digital_root(123));
}
