fn dig_pow(n: i64, p: i32) -> i64 {
    let vs: Vec<i64> = n
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let pow_inc: i64 = vs
        .iter()
        .enumerate()
        // .map(|(i, x)| x ^(2u32.pow((i as i32 + p)as u32)) as i64)
        .map(|(i, x)| (x.pow((i as i32 + p) as u32)) as i64)
        .collect::<Vec<_>>()
        .iter()
        .sum();

    let k = pow_inc / n;

    // println!("vs: {:?}", vs);
    println!("pow_inc: {:?}", pow_inc);
    // println!("ans: {:?}", pow_inc / n);
    println!("k: {:?}", k);
    if k * n != pow_inc {
        return -1;
    }
    k
    // p.into()
}

fn main() {
    println!("{:?}", dig_pow(89, 1));
    println!("{:?}", dig_pow(92, 1));
    println!("{:?}", dig_pow(695, 2));
    println!("{:?}", dig_pow(46288, 3));
}

// soln:
// fn dig_pow(n: i64, p: i32) -> i64 {
//     let r: i64 = n.to_string().chars()
//     .map(|c| (c as i64) - 48)
//     .enumerate()
//     .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
//     .sum();
//
//     match r%n {
//         0 => r/n,
//         _ => -1,
//     }
// }
//
// pub fn dig_pow(n: i64, p: i32) -> i64 {
//     let m = n.to_string();
//     match m.char_indices().fold(0_i64, |acc, (c, i)| {
//         let i = i.to_string().parse::<i64>().unwrap();
//         acc + i.pow(p as u32 + c as u32)
//     }) {
//         d  if d  % n == 0 => d  / n,
//         _ => -1
//     }
// }
//
// fn dig_pow(n: i64, p: i32) -> i64 {
//     let mut sm: i64 = 0; let mut m = n;
//     let mut pp: u32 = ((n as f64).log10() as u32) + (p as u32);
//     while m > 0 {
//         sm += (m % 10).pow(pp);
//         m = m / 10;
//         pp -= 1;
//     }
//     if sm % n == 0 {
//         return sm / n;
//     } else {
//         return -1;
//     }
// }
//
// fn dig_pow(n: i64, p: i32) -> i64 {
//     let sum : i64 = n.to_string().chars().enumerate()
//         .fold(0, |acc, (i,d)|
//             acc + (d as i64 - 48).pow(p as u32 + i as u32)
//         );
//
//     match sum % n {
//         0 => sum / n,
//         _ => -1
//     }
// }
//
// fn dig_pow(n: i64, p: i32) -> i64 {
//     let result: i64 = n.to_string().chars().map(|c| c as i64 - 48).enumerate().map(|(i, v)| v.pow(p as u32 + i as u32)).sum();
//
//     return match result % n {
//         0 => result / n,
//         _ => -1
//     }
// }
