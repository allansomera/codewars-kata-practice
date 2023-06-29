fn max_diff(numbers: &[i32]) -> i32 {
    if numbers.len() == 0 {
        return 0;
    }
    let mut c: Vec<i32> = numbers.clone().to_vec();
    c.sort();
    let max: i32 = *c.iter().max().unwrap();
    let min: i32 = *c.iter().min().unwrap();
    max - min
}

fn main() {
    println!("{:?}", max_diff(&[0, 1, 2, 3, 4, 5, 6]));
}

// soln:
// fn max_diff(numbers: &[i32]) -> i32 {
//     match (numbers.iter().min(), numbers.iter().max()) {
//         (Some(min), Some(max)) => max - min,
//         _ => 0,
//     }
// }
//
// fn max_diff(numbers: &[i32]) -> i32 {
//     numbers.iter().max().unwrap_or(&0) - numbers.iter().min().unwrap_or(&0)
// }
//
// fn max_diff(a: &[i32]) -> i32 {
//     if a.len()==0 {return 0;}
//     a.iter().max().unwrap()-a.iter().min().unwrap()
// }
//
// fn max_diff(numbers: &[i32]) -> i32 {
//     match numbers {
//         [] | [_] => 0,
//         x => x.iter().max().unwrap() - x.iter().min().unwrap()
//     }
// }
//
// fn max_diff(numbers: &[i32]) -> i32 {
//     let max = match numbers.iter().max() {
//         Some(a) => a,
//         None => &0
//     };
//
//     let min = match numbers.iter().min() {
//         Some(a) => a,
//         None => &0
//     };
//
//     max - min
// }
