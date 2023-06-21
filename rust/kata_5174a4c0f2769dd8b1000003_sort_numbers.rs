fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32>;
    if arr.is_empty() {
        v = Vec::new();
    } else {
        v = arr.clone();
        v.sort();
    }
    v
}

fn main() {
    println!("{:?}", sort_numbers(&vec![1, 2, 3, 10, 5]));
}

// soln:
// fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
//     let mut answer = arr.clone();
//     answer.sort();
//     answer
// }
//
// fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
//     let mut v = arr.clone();
//     v.sort_unstable();
//     v
// }
//
// fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
//     let mut sorted = arr.to_vec();
//     sorted.sort();
//     sorted
// }
//
// fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
//     let mut cpy = arr.clone();
//     cpy.sort();
//     cpy.to_vec()
// }
