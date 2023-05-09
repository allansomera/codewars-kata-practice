fn digitize(n: u64) -> Vec<u8> {
    let mut v: Vec<u8> = n
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    v.reverse();
    v
}

fn main() {
    println!("{:?}", digitize(348597));
}

// soln:
// fn digitize(n: u64) -> Vec<u8> {
//     n
//         .to_string()
//         .chars()
//         .map(|c| c.to_digit(10).unwrap() as u8)
//         .rev()
//         .collect::<Vec<u8>>()
// }
//
// fn digitize(n: u64) -> Vec<u8> {
//     n.to_string()
//      .chars()
//      .rev()
//      .map(|c| c as u8 - b'0')
//      .collect()
// }
//
// fn digitize(n: u64) -> Vec<u8> {
//     n.to_string().chars().rev().map(|x| x as u8 - 0x30).collect()
// }
//
// fn digitize(n: u64) -> Vec<u8> {
//     n.to_string()
//         .chars()
//         .map(|digit| digit.to_digit(10).unwrap() as u8)
//         .rev()
//         .collect()
// }
//
// use std::iter;
//
// fn digitize(x: u64) -> Vec<u8> {
//     iter::successors(Some(x), |&x| Some(x / 10).filter(|&x| x > 0))
//         .map(|x| (x % 10) as u8)
//         .collect()
// }
//
// fn digitize(n: u64) -> Vec<u8> {
//     n.to_string().chars().map(|x| x.to_digit(10).unwrap() as u8).rev().collect()
// }
//
// fn digitize(n: u64) -> Vec<u8> {
//     let mut result: Vec<u8> = n
//         .to_string()
//         .chars()
//         .filter_map(|x| Some(x.to_digit(10)? as u8))
//         .collect();
//     result.reverse();
//     result
// }
//
// fn digitize(n: u64) -> Vec<u8> {
//     n.to_string()
//         .chars()
//         .into_iter()
//         .rev()
//         .map(|digit| String::from(digit).parse::<u8>().unwrap())
//         .collect()
// }
