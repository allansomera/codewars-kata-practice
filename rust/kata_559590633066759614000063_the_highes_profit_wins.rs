fn min_max(lst: &[i32]) -> (i32, i32) {
    (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}

fn main() {
    println!("{:?}", min_max(&[1, 2, 3, 4, 5]));
}

// soln:
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     let min = lst.iter().min().unwrap();
//     let max = lst.iter().max().unwrap();
//
//     (*min, *max)
// }
//
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     lst.iter()
//     .fold((lst[0], lst[0]), |(min, max), &x| {
//         (if min > x { x } else { min }, if max < x { x } else { max })
//     })
// }
//
// use std::cmp::{max, min};
//
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     lst.iter().fold((i32::MAX, i32::MIN), |p, &x| (min(p.0, x), max(p.1, x)))
// }
//
// use itertools::Itertools;
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     lst.iter()
//         .copied()
//         .minmax()
//         .into_option()
//         .expect("Should never be empty")
// }
//
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     let mut arr = lst.to_vec();
//     arr.sort();
//
//     (arr[0], arr[arr.len() - 1])
// }
//
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     let iter = lst.iter();
//     (*iter.clone().min().unwrap(), *iter.clone().max().unwrap())
// }
//
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     (
//         *lst.iter().reduce(|a, b| {if (a < b) { a } else { b }}).unwrap(),
//         *lst.iter().reduce(|a, b| {if (a > b) { a } else { b }}).unwrap()
//     )
// }
//
// fn min_max(lst: &[i32]) -> (i32, i32) {
//     (lst.iter().min().copied().unwrap(), lst.iter().max().copied().unwrap())
// }
