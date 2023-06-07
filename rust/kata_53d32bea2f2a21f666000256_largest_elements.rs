fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
    let mut c = vec![0; xs.len()];
    c.clone_from_slice(&xs);
    let mut x = vec![0; n];
    c.sort();
    c.reverse();
    x.clone_from_slice(&c[..n]);
    x.reverse();
    x
}

fn main() {
    println!("{:?}", largest(2, &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1]));
}

// soln:
//
// fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
//   let mut xs = xs.to_vec();
//   xs.sort();
//
//   xs[xs.len() - n..].to_vec()
// }
//
// fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
//     let mut vec = xs.to_vec();
//     vec.sort();
//     vec[xs.len()-n..].to_vec()
// }
//
// fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
//     let mut v = xs.to_vec();
//     v.sort();
//     v.into_iter().skip(xs.len() - n).collect()
// }
//
// pub fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
//     let mut xs = xs.to_vec();
//     xs.sort();
//     xs.reverse();
//     xs.truncate(n);
//     xs.reverse();
//     xs
// }
//
// fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
//     let mut j = xs.to_owned();
//     j.sort();
//     j.reverse();
//     j.truncate(n);
//     j.reverse();
//     j
// }
//
