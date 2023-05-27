fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let mut b = vec![0; ages.len()];
    b.copy_from_slice(&ages);
    b.sort();
    b.reverse();
    let mut c = [0u8; 2];
    c.copy_from_slice(&b[0..2].iter().map(|&x| x as u8).collect::<Vec<u8>>());
    c.sort();
    c
}

fn main() {
    println!("{:?}", two_oldest_ages(&[1, 5, 87, 45, 8, 8]));
}

// soln:
// fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
//     let mut ages = ages.to_vec();
//     ages.sort();
//
//     [ages[ages.len() - 2], ages[ages.len() - 1]]
// }
//
// fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
//     let mut result = ages.to_owned();
//     result.sort();
//     result.reverse();
//     [result[1], result[0]]
// }
//
// fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
// 	let mut v = ages.to_vec();
// 	v.sort();
// 	let a = [v[v.len() - 2], v[v.len() - 1]];
// 	a
// }
