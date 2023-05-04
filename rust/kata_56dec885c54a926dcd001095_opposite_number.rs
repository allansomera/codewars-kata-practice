fn opposite(number: i32) -> i32 {
    number * -1
}

fn main() {
    println!("{}", opposite(-10));
}

// soln:
// fn opposite(number: i32) -> i32 {
//   -number
// }
//
// use std::ops::Neg;
//
// fn opposite(number: i32) -> i32 {
//   number.neg()
// }
//
// fn opposite(number: i32) -> i32 {
//   number.checked_neg().unwrap()
// }
//
// fn opposite(number: i32) -> i32 {
//   -1i32*number
// }
