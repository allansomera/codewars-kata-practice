fn square_sum(vec: Vec<i32>) -> i32 {
    vec.into_iter().map(|x| x * x).sum()
}

fn main() {
    println!("{:?}", square_sum(vec![1, 2]))
}

// soln:
// fn square_sum(vec: Vec<i32>) -> i32 {
//   vec.iter().fold(0, |t,i| t + i*i)
// }
//
// fn square_sum(vec: Vec<i32>) -> i32 {
//     vec.iter().map(|v| v.pow(2)).sum()
// }
//
// fn square_sum(vec: Vec<i32>) -> i32 {
//   vec.iter().fold(0, |acc, x| acc + x.pow(2))
// }
