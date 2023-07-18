fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    // println!("f0: {}\np: {}\nc0: {}\nn: {}\ni: {}", f0, p, c0, n, i);
    // true
    // let mut deposit: i32 = 0;
    // let total: i32 = 0;
    let mut withdraw: Vec<i32> = Vec::new();
    let mut amounts: Vec<i32> = Vec::new();

    amounts.push(f0.clone().to_owned());
    withdraw.push(c0.clone().to_owned());
    for _ in 1..=n {
        let curr_withdraw = withdraw[withdraw.len() - 1];
        let withdraw_next: i32 = (withdraw[withdraw.len() - 1] as f64
            + ((i / 100.0) * curr_withdraw as f64))
            .floor() as i32;
        let intr_f: i32 = (((p / 100.0) * amounts[amounts.len() - 1] as f64) - curr_withdraw as f64)
            .floor() as i32;
        let f: i32 = amounts[amounts.len() - 1] + intr_f;

        amounts.push(f);
        withdraw.push(withdraw_next)
    }
    let neg: Vec<(usize, &i32)> = amounts
        .iter()
        .enumerate()
        .filter(|(_, &num)| num < 0)
        .collect();
    println!("amounts: {:?}", amounts);
    println!("withdraw: {:?}", withdraw);
    println!("neg: {:?}", neg);
    // println!("{:?}", neg[0].0);

    if neg.len() == 0 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    // println!("{:?}", fortune(100_000, 1.0, 2_000, 15, 1.0));
    // println!("{:?}", fortune(100_000_000, 1.5, 10_000_000, 50, 1.0));
    println!("{:?}", fortune(100_000_000, 5.0, 1_000_000, 50, 1.0));
    println!("{:?}", fortune(1000, 1.0, 250, 5, 0.0));
    // println!("{:?}", fortune(100_000, 1.0, 9_185, 12, 1.0));
}
