fn positive_sum(slice: &[i32]) -> i32 {
    let sum: i32 = slice.iter().filter(|x| x.is_positive()).sum();
    sum
}

fn main() {
    println!("{:?}", positive_sum(&[1, 2, 3, 4, 5]));
}

// soln:
// fn positive_sum(xs: &[i32]) -> i32 {
//     xs.iter().filter(|&&x| x > 0).sum()
// }
//
// fn positive_sum(slice: &[i32]) -> i32 {
//     slice.iter().filter(|&x| x > &0).sum::<i32>()
// }
//
// fn positive_sum(slice: &[i32]) -> i32 {
//   slice
//     .iter()
//     .map(|&x| x)
//     .filter(num::Signed::is_positive)
//     .sum()
// }
//
// fn positive_sum(slice: &[i32]) -> i32 {
//     slice
//     .iter()
//     .map(|x| if x > &0 {&x} else {&0})
//     .sum()
// }
