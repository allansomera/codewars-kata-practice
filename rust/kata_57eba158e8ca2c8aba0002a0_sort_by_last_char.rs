fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut a: Vec<String> = s
        .split_whitespace()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    a.sort_by(|a, b| {
        a.chars()
            .rev()
            .next()
            .unwrap()
            .cmp(&b.chars().rev().next().unwrap())
    });
    a
}

fn main() {
    println!("{:?}", sort_by_last_char("this is a sentence"));
}

// soln:
// fn sort_by_last_char(s: &str) -> Vec<String> {
//     let mut v = s.split(' ').map(|s|s.to_string()).collect::<Vec<_>>();
//     v.sort_by_key(|k|k.chars().last());
//     v
// }
//
// fn sort_by_last_char(s: &str) -> Vec<String> {
// 	let mut vec: Vec<_> = s.split_ascii_whitespace().map(String::from).collect();
//
// 	vec.sort_by_key(|a| a.bytes().last().unwrap());
//
// 	vec
// }
//
// // ==
// use itertools::Itertools;
// fn sort_by_last_char(s: &str) -> Vec<String> {
//     s
//     .split_ascii_whitespace()
//     .sorted_by(|a, b| {
//         a.chars().last().unwrap().cmp(&b.chars().last().unwrap())
//     })
//     .map(|x| x.to_string())
//     .collect::<Vec<String>>()
// }
// // ==
// pub fn sort_by_last_char(s: &str) -> Vec<String> {
//     let mut v: Vec<String> = s.split_whitespace().map(|x| x.to_string()).collect();
//     v.sort_by_key(|a| a.chars().last().unwrap());
//     v
// }
//
// fn sort_by_last_char(s: &str) -> Vec<String> {
//     let mut res = s
//         .split_whitespace()
//         .map(|x| x.to_string())
//         .collect::<Vec<_>>();
//
//     res.sort_by(|a,b| a.chars().last().unwrap().cmp(&b.chars().last().unwrap()));
//
//     res
// }
//
// fn sort_by_last_char(s: &str) -> Vec<String> {
//     let mut words = s.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
//
//     words.sort_by(|a, b| {
//         let a_last = a.chars().last().unwrap();
//         let b_last = b.chars().last().unwrap();
//
//         a_last.cmp(&b_last)
//     });
//
//     words
// }
//
// fn sort_by_last_char(s: &str) -> Vec<String> {
//     let mut vector = s.split_whitespace()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>();
//
//     vector.sort_by(|a, b| a.chars().last().unwrap().cmp(&b.chars().last().unwrap()));
//
//     vector
// }
