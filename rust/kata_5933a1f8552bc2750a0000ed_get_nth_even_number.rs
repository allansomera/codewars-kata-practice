fn nth_even(n: u32) -> u32 {
    if n == 1 {
        return 0;
    };
    let ans: u32 = (n * 2) - 2;
    ans
}

fn main() {
    println!("{:?}", nth_even(3))
}

// soln:
// fn nth_even(n: u32) -> u32 {
//     (n - 1) * 2
// }
//
// fn nth_even(n: u32) -> u32 {
//     2 * (n - 1)
// }
//
// fn nth_even(n: u32) -> u32 {
//     n * 2 - 2
// }
//
// fn nth_even(n: u32) -> u32 {
//   match n { 1 => 0, _ => 2 * (n-1) }
// }
//
// fn nth_even(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     }
//     (n - 1) * 2
// }
//
// fn nth_even(n: u32) -> u32 {
//     let mut counter = 0;
//     (0..n - 1).for_each(|_| counter += 2);
//     counter
// }
//
// fn nth_even(n: u32) -> u32 {
//     n+n-2
// }
//
// fn nth_even(n: u32) -> u32 {
//      (n << 1) - 2
// }
//
// fn nth_even(n: u32) -> u32 {
//     (n as i64 - 1).max(0) as u32 * 2
// }
