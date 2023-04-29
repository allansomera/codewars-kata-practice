fn invert(values: &[i32]) -> Vec<i32> {
    let vec: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(_, &el)| if el > 0 { el * -1 } else { i32::abs(el) })
        .collect();
    vec
}

fn main() {
    println!("{:?}", invert(&vec![-1, 2, 3, 4, 5]));
}

// soln:
// fn invert(values: &[i32]) -> Vec<i32> {
//     values.iter().map(|x| -x).collect()
// }
//
// fn invert(values: &[i32]) -> Vec<i32> {
//     values.iter().map(std::ops::Neg::neg).collect()
// }
//
// fn invert(values: &[i32]) -> Vec<i32> {
//     values.iter().map(|v| {-v}).collect()
// }
//
// fn invert(values: &[i32]) -> Vec<i32> {
//      values.iter().map(|i| *i * -1).collect()
// }
