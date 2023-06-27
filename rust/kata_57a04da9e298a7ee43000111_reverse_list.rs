fn reverse_list(lst: &[i32]) -> Vec<i32> {
    let a: Vec<i32> = lst.iter().cloned().rev().collect::<Vec<i32>>();
    a
}

fn main() {
    println!("{:?}", reverse_list(&[1, 2, 3, 4]));
}

// soln:
// fn reverse_list(lst: &[i32]) -> Vec<i32> {
//     lst.iter().rev().cloned().collect()
// }
//
// fn reverse_list(lst: &[i32]) -> Vec<i32> {
//     let mut out = lst.to_vec();
//     out.reverse();
//     out
// }
//
// fn reverse_list(lst: &[i32]) -> Vec<i32> {
//     lst.iter().map(|x| *x).rev().collect()
// }
//
// fn reverse_list(a: &[i32]) -> Vec<i32> {
//     a.iter().map(|e| *e).rev().collect::<Vec<i32>>()
// }
