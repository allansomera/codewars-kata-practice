fn get_average(marks: &[i32]) -> i32 {
    let avg: i32 = marks.iter().sum::<i32>() / (marks.len() as i32);
    avg
}

fn main() {
    println!(
        "{:?}",
        get_average(&[1, 2, 15, 15, 17, 11, 12, 17, 17, 14, 13, 15, 6, 11, 8, 7])
    );
}

// soln:
// fn get_average(m: &[f32]) -> f32 {
//     (m.iter().sum::<f32>()/m.len() as f32).floor()
// }
//
// fn get_average(marks: &[i32]) -> i32 {
//     marks.iter().sum::<i32>() / marks.len() as i32
// }
//
// fn get_average(marks: &[f32]) -> f32 {
//     if marks.len() == 0 {
//         panic!();
//     }
//     (marks.iter().sum::<f32>() / (marks.len() as f32)).floor()
// }
//
// fn get_average(marks: &[f32]) -> f32 {
//     let sum: f32 = marks.iter().sum();
//     return ((sum / (marks.len() as f32)) as i32) as f32;
// }
//
// fn get_average(marks: &[f32]) -> f32 {
//     (marks.iter().cloned().sum::<f32>() / (marks.len() as f32)).floor()
// }
//
// fn get_average(marks: &[i32]) -> i32 {
//     let s: &i32 = &marks.iter().sum();
//     let r: i32 = (s / &marks.len().try_into().unwrap()).try_into().unwrap();
//     r
// }
//
// fn get_average(marks: &[i32]) -> i32 {
//     marks.iter().sum::<i32>() as i32 / marks.len() as i32
// }
//
// fn get_average(marks: &[i32]) -> i32 {
//     marks.iter().fold(0, |i,j| i + j) / marks.len() as i32
// }
