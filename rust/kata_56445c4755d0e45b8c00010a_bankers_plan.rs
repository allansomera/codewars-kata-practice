fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    // println!("f0: {}\np: {}\nc0: {}\nn: {}\ni: {}", f0, p, c0, n, i);
    // true
    // let mut deposit: i32 = 0;
    let total: i32 = 0;
    let mut withdraw: Vec<i32> = Vec::new();
    let mut amounts: Vec<i32> = Vec::new();

    amounts.push(f0.clone().to_owned());
    withdraw.push(c0.clone().to_owned());
    for x in 1..=n {
        let intr_f: i32 = ((p / 100.0) * amounts[amounts.len() - 1] as f64) as i32;
        let f: i32 = amounts[amounts.len() - 1] + intr_f;
        let withdraw_next: i32 = ((i / 100.0) * withdraw[withdraw.len() - 1] as f64) as i32;
        amounts.push(f);
        withdraw.push(withdraw_next)
    }
    println!("amounts: {:?}", amounts);
    println!("withdraw: {:?}", withdraw);
    true
}

fn main() {
    println!("{:?}", fortune(100_000, 1.0, 2_000, 15, 1.0));
}
