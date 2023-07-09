fn capitalize(s: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut s1: Vec<char> = s.chars().collect();
    let mut s2: Vec<char> = s.chars().collect();
    (0..s.len())
        .step_by(2)
        .for_each(|x| s1[x] = s1[x].to_uppercase().next().unwrap());
    v.push(s1.into_iter().collect::<String>());

    (1..s.len())
        .step_by(2)
        .for_each(|x| s2[x] = s2[x].to_uppercase().next().unwrap());

    v.push(s2.into_iter().collect::<String>());
    v
}

fn main() {
    println!("{:?}", capitalize("abcdef"))
}

// soln:
// fn capitalize(s: &str) -> Vec<String> {
//     let (left, right) : (String, String) = s.chars().enumerate().map(|(i,c)|
//         {if i % 2 == 0 {
//             (c.to_ascii_uppercase(), c)
//         } else {
//             (c.to_ascii_lowercase(), c.to_ascii_uppercase())
//     }}).unzip();
//     vec![left,right]
// }
//
// fn capitalize(s: &str) -> Vec<String> {
//     vec![
//         s.chars().enumerate().map(|(i, c)| if i % 2 == 0 { c.to_ascii_uppercase() } else { c } ).collect(),
//         s.chars().enumerate().map(|(i, c)| if i % 2 == 1 { c.to_ascii_uppercase() } else { c } ).collect(),
//     ]
// }
//
// fn capitalize(s: &str) -> Vec<String> {
//     let shuffle = |odd: usize| -> String {
//         s.chars()
//             .enumerate()
//             .map(|(i, c)| match i {
//                 _ if i % 2 == odd => c.to_ascii_uppercase(),
//                 _ => c.to_ascii_lowercase(),
//             })
//             .collect()
//     };
//
//     vec![shuffle(0), shuffle(1)]
// }
//
// fn capitalize(s: &str) -> Vec<String> {
//     let iter = s.chars().enumerate();
//     vec![
//         iter.clone().map(|(i, c)| if (i % 2) == 0 {c.to_ascii_uppercase()} else {c}).collect(),
//         iter.clone().map(|(i, c)| if (i % 2) == 1 {c.to_ascii_uppercase()} else {c}).collect(),
//     ]
// }
//
// fn capitalize(s: &str) -> Vec<String> {
//     vec![
//         s.char_indices().map(|e| if e.0%2 == 0 {e.1.to_uppercase().to_string()} else {e.1.to_string()}).collect::<String>(),
//         s.char_indices().map(|e| if e.0%2 != 0 {e.1.to_uppercase().to_string()} else {e.1.to_string()}).collect::<String>()
//     ]
// }
//
// fn capitalize(s: &str) -> Vec<String> {
//     let even = s.chars()
//         .enumerate()
//         .map(|(i, c)| if i % 2 == 0 { c.to_uppercase().next().unwrap() } else { c })
//         .collect();
//     let odd = s.chars()
//         .enumerate()
//         .map(|(i, c)| if i % 2 == 1 { c.to_uppercase().next().unwrap() } else { c })
//         .collect();
//     vec![even, odd]
// }
