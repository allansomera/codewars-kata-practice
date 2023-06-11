use std::collections::VecDeque;
fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut vv: Vec<(char, i32)> = Vec::new();
    let mut char_deque: VecDeque<_> = VecDeque::new();
    for c in sip.chars() {
        char_deque.push_back((c, 1));
    }

    for (c, _) in char_deque.into_iter() {
        if let Some((_, count)) = vv.iter_mut().find(|(char, _)| *char == c) {
            *count += 1;
        } else {
            vv.push((c, 1));
        }
    }
    vv
}

fn main() {
    println!("{:?}", ordered_count("abracadabra"));
}

// soln:
// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     sip.chars().fold(Vec::new(), |mut acc, c| {
//         match acc.iter().position(|it| it.0 == c) {
//             Some(idx) => acc[idx].1 += 1,
//             None => acc.push((c, 1)),
//         };
//         acc
//     })
// }
//
// use itertools::Itertools;
// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     sip.chars().unique().map(|c| (c, sip.matches(c).count() as i32)).collect()
// }
//
// use std::collections::HashMap;
//
// fn ordered_count(count_me: &str) -> Vec<(char, i32)> {
//     let mut order: Vec<char> = vec![];
//     let mut counts: HashMap<char, i32> = HashMap::new();
//
//     count_me.chars().for_each(|c| {
//         if let Some(n) = counts.get_mut(&c) {
//             *n += 1;
//         } else {
//             counts.insert(c, 1);
//             order.push(c);
//         }
//     });
//
//     order
//         .iter()
//         .map(|c| (c.to_owned(), counts.get(&c).unwrap().to_owned()))
//         .collect::<Vec<_>>()
// }
//
// use itertools::Itertools;
//
// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     sip
//         .chars()
//         .into_iter()
//         .unique()
//         .map(|c| (c, sip.matches(c).count() as i32))
//         .collect()
// }
//
// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     sip.chars().fold(Vec::new(), |mut acc, c| {
//         if let Some(pos) = acc.iter().position(|pair| pair.0 == c) {
//             acc[pos].1 += 1;
//         } else {
//             acc.push((c, 1));
//         }
//         acc
//     })
// }
//
// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     let input = sip.chars().fold(vec![], |mut rs, x|{
//         if !rs.contains(&x) {
//             rs.push(x);
//         }
//         rs
//     });
//     input.iter().map(|&x| (x, sip.matches(x).count() as i32)).collect()
// }
//
// use std::collections::HashMap;
//
// fn ordered_count(sip: &str) -> Vec<(char, i32)> {
//     let mut counts = HashMap::new();
//     let mut vals = Vec::new();
//     for v in sip.chars() {
//         let entry = counts.entry(v).or_insert_with_key(|k|{
//             vals.push(*k);
//             0
//         });
//         *entry += 1;
//     }
//     vals.iter().map(|v|(*v, *counts.get(v).unwrap())).collect()
// }
