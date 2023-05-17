fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0 as f64;
    }
    let res: f64 = (slice.iter().sum::<f64>()) / (slice.len() as f64);
    res
}

fn main() {
    println!(
        "{:?}",
        find_average(&[
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0
        ])
    );
}

// soln:
// fn find_average(xs: &[f64]) -> f64 {
//     match xs.len() {
//         0 => 0.,
//         n => xs.iter().sum::<f64>() / n as f64
//     }
// }
//
// fn find_average(slice: &[f64]) -> f64 {
//     let total: f64 = slice.iter().sum();
//     if slice.len() > 0 {
//         return total / slice.len() as f64;
//     }
//     0.0
// }
//
// pub fn find_average(slice: &[f64]) -> f64 {
//     if slice.is_empty() {
//         0.0
//     } else {
//         slice.iter().sum::<f64>() / slice.len() as f64
//     }
// }
//
// fn find_average(slice: &[f64]) -> f64 {
//     slice.iter().sum::<f64>() / slice.len().max(1) as f64
// }
//
// fn find_average(slice: &[f64]) -> f64 {
//     let n = slice.len();
//     if n == 0 {
//         return 0f64;
//     }
//     slice.into_iter().sum::<f64>() / ( n as f64 )
// }
//
// fn find_average(slice: &[f64]) -> f64 {
//
//     match slice.is_empty(){
//         true => 0.0,
//         false => slice.iter().sum::<f64>() / slice.len() as f64,
//         _ => 0.0
//     }
// }
