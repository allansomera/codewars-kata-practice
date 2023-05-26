// use std::collections::HashSet;

fn remove_duplicate_words(s: &str) -> String {
    // let ss: String = s
    //     .split_whitespace()
    //     .into_iter()
    //     .collect::<HashSet<_>>()
    //     .into_iter()
    //     .collect::<Vec<_>>()
    //     .join(" ");
    // ss

    let ss: Vec<String> = s
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();

    let mut elem: Vec<String> = Vec::new();
    for x in ss.into_iter() {
        if !elem.contains(&x) {
            elem.push(x.to_string());
        }
    }
    elem.join(" ")
}

fn main() {
    println!(
        "{:?}",
        remove_duplicate_words(
            "alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta",
        ),
    );
}

// soln:
// use std::collections::HashSet;
//
// fn remove_duplicate_words(s: &str) -> String {
//     let mut words: HashSet<&str> = HashSet::new();
//
//     s.split_whitespace()
//         .filter(|w| words.insert(w))
//         .collect::<Vec<&str>>()
//         .join(" ")
// }
//
// extern crate itertools;
// use itertools::Itertools;
// use std::collections::HashSet;
//
// fn remove_duplicate_words(s: &str) -> String {
//     s.split_whitespace().unique().join(" ")
// }
//
// use std::collections::HashSet;
//
// fn remove_duplicate_words(s: &str) -> String {
//     let mut v: Vec<&str> = vec![];
//
//     for s in s.split_whitespace() { if !v.contains(&s) { v.push(s); } }
//
//     v.join(" ")
// }
//
// use std::collections::HashSet;
//
// fn remove_duplicate_words(s: &str) -> String {
//   let mut set: HashSet<&str> = HashSet::new();
//   return s.split_whitespace().filter(|word| {
//     if set.contains(word) {
//       return false;
//     }
//     set.insert(word);
//     return true;
//   }).collect::<Vec<_>>().join(" ");
// }
//
// fn remove_duplicate_words(s: &str) -> String {
//     let mut cache = std::collections::HashSet::new();
//
//     s.split_whitespace().filter(|w| cache.insert(w.to_string())).collect::<Vec<_>>().join(" ")
// }
//
// use std::collections::HashSet;
//
// fn remove_duplicate_words(s: &str) -> String {
//     let mut res = Vec::<&str>::new();
//     let words = s.split(" ").collect::<Vec<_>>();
//     for word in words.iter() {
//         if !res.contains(word) {
//             res.push(word);
//         }
//     }
//     res.join(" ")
// }
//
// use std::collections::HashSet;
//
// fn remove_duplicate_words(s: &str) -> String {
//     let word_arr: Vec<&str> = s.split_whitespace().collect();
//
//     let x: Vec<&str> = word_arr
//         .iter()
//         .enumerate()
//         .filter(|&(i, z)| i == word_arr
//             .iter()
//             .position(|&b| *b == **z)
//             .unwrap())
//         .map(|(_, e)| *e)
//         .collect();
//
//     x.join(" ")
// }
