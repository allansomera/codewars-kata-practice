fn find_short(s: &str) -> u32 {
    let v: Vec<_> = s.split_whitespace().map(|x| x.len()).collect();
    let (len_idx, _) = v
        .iter()
        .enumerate()
        .min_by(|(_, &a), (_, &b)| a.cmp(&b))
        .unwrap();
    // println!("{:?}", v);
    // println!("{:?}", len_idx);
    v[len_idx] as u32
}

fn main() {
    println!(
        "{:?}",
        find_short("bitcoin take over the world maybe who knows perhaps")
    );
}

// soln:
// fn find_short(s: &str) -> u32 {
//   s.split_whitespace()
//    .map(|word| word.len())
//    .min()
//    .unwrap_or(0) as u32
// }
//
// fn find_short(s: &str) -> u32 {
//   match s.split_whitespace().map(|s| s.len()).min() {
//     Some(min) => min as u32,
//     None => 0u32,
//   }
// }
//
// fn find_short(s: &str) -> u32 {
//   s
//     .split_whitespace()
//     .min_by_key(|s| s.len())
//     .unwrap()
//     .len() as u32
// }
//
// fn find_short(s: &str) -> u32 {
//   s.split_whitespace().map(&str::len).min().unwrap() as u32
// }
//
// fn find_short(s: &str) -> u32 {
//     s.split_ascii_whitespace()
//         .fold(s.len() as u32, |acc, x| acc.min(x.len() as u32))
// }
//
// fn find_short(s: &str) -> u32 {
//     let mut q = s.split_whitespace().collect::<Vec<&str>>();
//     q.sort_by(|x, y| x.len().cmp(&y.len()));
//     q[0].len() as u32
// }
