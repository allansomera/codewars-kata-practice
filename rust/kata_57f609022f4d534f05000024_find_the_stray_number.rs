use std::collections::HashMap;

fn stray(arr: &[u32]) -> u32 {
    let mut count = HashMap::new();
    let mut single = 0;
    for c in arr.into_iter() {
        *count.entry(c).or_insert(0) += 1;
    }
    for (&&k, &v) in count.iter() {
        if v == 1 {
            single = k;
        }
    }
    single
}

fn main() {
    println!("{:?}", stray(&[1, 1, 1, 1, 1, 1, 2]));
}

// soln:
// fn stray(arr: &[u32]) -> u32 {
//     arr.iter().fold(0, |acc, el| acc ^ el)
// }
//
// fn stray(xs: &[u32]) -> u32 {
//     if xs[0] != xs[1] {
//         return if xs[1] == xs[2] { xs[0] } else { xs[1] };
//     }
//     *xs[2..].into_iter().find(|x| **x != xs[0]).unwrap()
// }
//
// fn stray(arr: &[u32]) -> u32 {
//     let mut result = 0;
//
//     for &num in arr {
//         if arr.iter().filter(|&n| *n == num).count() == 1 {
//             result = num;
//             break;
//         }
//     }
//
//     result
// }
//
// fn stray(arr: &[u32]) -> u32 {
//     arr.iter().fold(0, |a, b| a ^ b)
// }
//
// use std::collections::HashMap;
//
// fn stray(arr: &[u32]) -> u32 {
//     let mut map = HashMap::new();
//
//     for &n in arr {
//         map.entry(n).and_modify(|c| *c += 1).or_insert(1);
//     }
//
//     *map.iter().filter(|(_, &c)| c == 1).collect::<Vec<_>>()[0].0
// }
