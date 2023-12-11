fn dup(arry: Vec<String>) -> Vec<String> {
    let mut n_vec: Vec<String> = Vec::new();
    for s in &arry {
        let mut result = String::new();
        let mut prev_char: Option<char> = None;
        for c in s.chars() {
            if prev_char != Some(c) {
                result.push(c);
            }
            prev_char = Some(c);
        }
        n_vec.push(result.clone());
    }
    n_vec
}

fn main() {
    println!(
        "{:?}",
        dup(vec![
            "abracadabra".to_string(),
            "allottee".to_string(),
            "assessee".to_string()
        ])
    );
}

// soln:
// use itertools::Itertools;
//
// fn dup(ss: Vec<String>) -> Vec<String> {
//     ss.iter().map(|s| s.chars().dedup().collect()).collect()
// }
//
// fn dup(strings: Vec<String>) -> Vec<String> {
//     strings.iter()
//         .map(|string| string.as_str().chars().collect::<Vec<_>>())
//         .map(|mut chars| { chars.dedup(); chars.iter().collect() })
//         .collect()
// }
//
// fn dup(arry: Vec<String>) -> Vec<String> {
//     (0..arry.len())
//     .map(|i| {
//         let mut v = arry[i].chars().collect::<Vec<_>>();
//         v.dedup();
//         v.iter().collect::<String>()
//     })
//     .collect()
// }
//
// use itertools::Itertools;
//
// fn dup(arry: Vec<String>) -> Vec<String> {
//     arry.iter()
//         .map(|s|
//             s.chars()
//              .group_by(|&x|x)
//              .into_iter()
//              .map(|(k,_)|k)
//              .collect()
//         ).collect()
// }
//
//
