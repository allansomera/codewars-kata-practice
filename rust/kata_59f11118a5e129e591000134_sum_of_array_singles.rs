use std::collections::HashMap;

fn repeats(arr: &Vec<i32>) -> i32 {
    let mut count: HashMap<i32, usize> = HashMap::new();
    for &x in arr.iter() {
        *count.entry(x).or_insert(0) += 1;
    }

    let uniq: Vec<i32> = arr.iter().filter(|&&x| count[&x] == 1).cloned().collect();
    // println!("{:?}", uniq)
    uniq.iter().sum()
}

fn main() {
    println!("{:?}", repeats(&vec![4, 5, 7, 5, 4, 8]));
}

// soln:
// use itertools::Itertools;
//
// fn repeats(arr: &Vec<i32>) -> i32 {
//     (2 * arr.iter().unique().sum::<i32>()) - arr.iter().sum::<i32>()
// }
//
// use std::collections::HashMap;
//
// fn repeats(arr: &Vec<i32>) -> i32 {
//     let mut seen = HashMap::<i32, usize>::new();
//
//     for &n in arr.iter() {
//         *seen.entry(n).or_default() += 1;
//     }
//
//     seen.iter()
//         .filter(|&(_, &count)| count == 1)
//         .map(|(x, _)| x)
//         .sum()
// }
//
// fn repeats(arr: &Vec<i32>) -> i32 {
//     arr.iter().enumerate().fold(0, |acc, (i, x)| {
//         if arr.iter().position(|a| a == x) != Some(i) {
//             acc - x
//         } else {
//             acc + x
//         }
//     })
// }
//
// fn repeats(arr: &Vec<i32>) -> i32 {
//     let mut arr2 = arr.to_owned();
//     arr2.sort();
//     arr2.dedup();
//     let s = arr.iter().sum::<i32>();
//     s - ((s - arr2.iter().sum::<i32>()) << 1)
// }
//
// use std::collections::HashSet;
//
// fn repeats(arr: &Vec<i32>) -> i32 {
//     arr.iter().fold(
//         HashSet::new(),
//         |mut found, num| {
//             if found.contains(num) { found.remove(num); } else { found.insert(*num); }
//             found
//         }
//     ).iter().sum::<i32>()
// }
//
// fn repeats(arr: &Vec<i32>) -> i32 {
//     arr
//         .into_iter()
//         .filter(|x| arr.into_iter().filter(|y| y == x).count() == 1)
//         .collect::<Vec<_>>()
//         .into_iter()
//         .sum()
// }
