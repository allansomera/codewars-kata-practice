fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
    let mut flat: Vec<i32> = arr.iter().flatten().copied().collect();
    flat.sort();
    flat
}

fn main() {
    println!(
        "{:?}",
        flatten_and_sort(&[vec![3, 2, 1], vec![7, 9, 8], vec![6, 4, 5]])
    );
}

// soln:
// use itertools::Itertools;
//
// fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
//     arr.iter()
//        .flatten()
//        .copied()
//        .sorted()
//        .collect()
// }
//
// fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
//     let mut flattened: Vec<i32> = arr.iter().flatten().cloned().collect();
//     flattened.sort();
//     flattened
// }
//
// fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
//     let mut res = Vec::new();
//     for i in arr { for j in i { res.push(*j); } }
//     res.sort();
//     res
//
// }
//
// fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
//     let mut flat_array: Vec<i32> = arr
//     .iter()
//     .flat_map(|array| array.clone())
//     .collect();
//
//     flat_array.sort();
//     flat_array
// }
//
// use itertools::Itertools;
//
// fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
//     arr.iter().flatten().cloned().sorted().collect()
// }
//
// fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
//     let mut local_vec = arr.to_vec().into_iter().flatten().collect::<Vec<i32>>();
//     local_vec.sort();
//     local_vec
// }
