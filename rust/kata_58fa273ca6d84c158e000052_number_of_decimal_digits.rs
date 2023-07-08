fn digits(n: u64) -> usize {
    n.to_string().len() as usize
}

fn main() {
    println!("{:?}", digits(123123123));
}

// soln:
// fn digits(n: u64) -> usize {
//   n.to_string().len()
// }
//
// fn digits(n: u64) -> usize {
//   n.to_string().chars().count()
// }
//
// fn digits(n: u64) -> usize {
//     if n == 0 {
//         1
//     } else {
//         std::iter::successors(Some(n), |&n| if n == 0 { None } else { Some(n / 10) }).count() - 1
//     }
// }
//
// fn digits(n: u64) -> usize {
//   String::from(format!("{n}")).chars().count()
// }
