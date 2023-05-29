fn longest(a1: &str, a2: &str) -> String {
    let mut ch_a: Vec<char> = Vec::new();
    // let mut ch_b: Vec<char> = Vec::new();
    for c in a1.chars() {
        if !ch_a.contains(&c) {
            ch_a.push(c);
        }
    }
    for c in a2.chars() {
        if !ch_a.contains(&c) {
            ch_a.push(c);
        }
    }
    ch_a.sort();

    println!("ch_a: {:?}", ch_a);

    let str_a: String = ch_a.iter().map(|x| x.to_string()).collect::<String>();
    println!("str_a: {}", str_a);
    str_a
}

fn main() {
    println!(
        "{:?}",
        longest("loopingisfunbutdangerous", "lessdangerousthancoding")
    );
}

// soln:
// use std::collections::BTreeSet;
// fn longest(a1: &str, a2: &str) -> String {
//     a1.chars()
//         .chain(a2.chars())
//         .collect::<BTreeSet<char>>()
//         .iter()
//         .collect()
// }
//
// fn longest(a1: &str, a2: &str) -> String {
//     let mut res: Vec<_> = a1.chars().collect();
//     res.extend(a2.chars());
//     res.sort();
//     res.dedup();
//     res.into_iter().collect()
// }
//
// use itertools::Itertools;
//
// fn longest(a1: &str, a2: &str) -> String {
//     format!("{}{}", a1, a2).chars().sorted().dedup().collect()
// }
//
// fn longest(a1: &str, a2: &str) -> String {
//     ('a'..='z')
//         .filter( |c| (a1.contains(*c) || a2.contains(*c)) )
//         .collect()
// }
//
// fn longest(a1: &str, a2: &str) -> String {
//     let mut chars: Vec<char> = a1.chars().chain(a2.chars()).collect();
//     chars.sort();
//     chars.dedup();
//     chars.into_iter().collect()
// }
//
// fn longest(a1: &str, a2: &str) -> String {
//     let mut letters: Vec<char> = a1.chars().chain(a2.chars()).collect();
//     letters.sort();
//     letters.dedup();
//     letters.into_iter().collect()
// }
//
// fn longest(a1: &str, a2: &str) -> String {
//     ('a'..='z')
//         .filter(|c| (a1.to_string()+a2).contains(*c))
//         .collect::<String>()
// }
