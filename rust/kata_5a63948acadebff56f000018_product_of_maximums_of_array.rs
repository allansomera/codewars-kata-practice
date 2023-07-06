fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
    lst.sort_by(|a, b| b.cmp(a));
    lst.iter().take(n_largest_elements as usize).product()
    // println!("{:?}", lst);
}

fn main() {
    println!("{:?}", max_product(vec![4, 3, 5], 2));
}

// soln:
// fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
//     let i = lst.len() - n_largest_elements as usize;
//     lst.select_nth_unstable(i);
//     lst[i..].iter().product()
// }
//
// fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
//     lst.sort_by_key(|k| -k);
//     lst[0..n_largest_elements as usize].iter().product()
// }
//
// fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
//     lst.sort();
//     lst[lst.len()-n_largest_elements as usize..].into_iter().product()
// }
//
// fn max_product(mut v: Vec<i32>, n: i32) -> i32 {
//     v.sort();
//     v[v.len() - n as usize..].iter().product()
// }
//
// fn max_product(mut lst: Vec<i32>, n_largest_elements: i32) -> i32 {
//     lst.sort_by(|a, b| b.cmp(a));
//
//     lst
//         .iter()
//         .take(n_largest_elements as usize)
//         .fold(1, |a, e| a*e)
// }
