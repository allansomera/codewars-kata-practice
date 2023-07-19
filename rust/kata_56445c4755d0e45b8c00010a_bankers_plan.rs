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

    amounts.truncate(amounts.len() - 1);
    let neg: Vec<(usize, &i32)> = amounts
        .iter()
        .enumerate()
        .filter(|(_, &num)| num < 0)
        .collect();
    // println!("amounts: {:?}", amounts);
    // println!("withdraw: {:?}", withdraw);
    for (i, x) in amounts.iter().enumerate() {
        println!("year=>{}: {}", i, x);
    }
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

// soln:
// fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
//     let p = p * 0.01 + 1.0;
//     let i = i * 0.01 + 1.0;
//     let f0 = f0 as f64;
//     let c0 = c0 as f64;
//     (1..n).fold((f0, c0), |(f0, c0), _| ((p * f0 - c0).floor(), (i * c0).floor())).0 >= 0.0
// }
//
// fn fortune(mut f: i32, p: f64, mut c: i32, n: i32, i: f64) -> bool {
//     for _ in 1..n {
//         f += (f as f64 * (p / 100.0)) as i32 - c;
//         c += (c as f64 * (i / 100.0)) as i32;
//
//         if f < 0 {
//             return false;
//         }
//     }
//
//     true
// }
//
// fn fortune(f0: i32, p0: f64, c0: i32, n: i32, i0: f64) -> bool {
//     let mut f = f0 as f64;
//     let mut c = c0 as f64;
//     let mut p = (p0 as f64)/100.0;
//     let mut i = (i0 as f64)/100.0;
//     for _ in 1..n {
//         f = (f+p*f-c).floor();
//         c = (c+c*i).floor();
//     }
//     f>=0.0
// }
//
// fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
//     let mut deposit = f0;
//     let inf_coeff = 1.0 + i / 100.0;
//
//     for j in 1..n {
//         let withdraw = (c0 as f64 * inf_coeff.powi(j - 1)) as i32;
//         deposit = (deposit + (deposit as f64 * p / 100.0) as i32 - withdraw);
//     }
//     return deposit >= 0;
// }
//
// fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
//     (1..n)
//         .fold((f0 as f64, c0 as f64), |(f, c), _| {
//             (
//                 f + (f * (p / 100_f64) - c).floor(),
//                 c + (c * (i / 100_f64)).floor(),
//             )
//         })
//         .0
//         >= 0_f64
// }
//
// fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
//     let mut f = Vec::new();
//     let mut c = Vec::new();
//
//     f.push(f0);
//     c.push(c0);
//
//     for a in 1..(n as usize) {
//         let prev_f = f[a-1] as f64;
//         let prev_c = c[a-1] as f64;
//
//         f.push( ((1.0 + p*0.01)*prev_f - prev_c) as i32 );
//         c.push( ((1.0 + i*0.01)*prev_c) as i32 );
//     }
//
//     return f[f.len() - 1] >= 0
// }
//
// fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
//     let mut f = f0 as f64;
//     let mut c = c0 as f64;
//     let mut m = n - 1;
//     let pp = p / 100.0;
//     let ii = i / 100.0;
//     while m > 0 {
//         m = m - 1;
//         f = f + (pp * f - c).floor();
//         c = c + (ii * c).floor();
//     }
//     return f >= 0.0;
// }
