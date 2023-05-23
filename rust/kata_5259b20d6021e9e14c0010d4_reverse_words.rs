fn reverse_words(str: &str) -> String {
    let mut s: String = str
        .split_whitespace()
        .map(str::to_string)
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<String>();
    for (i, ch) in str.chars().enumerate() {
        if ch.is_whitespace() {
            s.insert(i, ch);
        }
    }
    s
}

fn main() {
    println!("{:?}", reverse_words("reverse  me"));
}

// soln:
// fn reverse_words(str: &str) -> String {
//     str.to_string()
//       .split(" ")
//       .map(|sub| sub.chars().rev().collect())
//       .collect::<Vec<String>>()
//       .join(" ")
// }
//
// fn reverse_words(str: &str) -> String {
//     str.to_string().split(" ")
//         .map(|word| word.chars().rev().collect::<String>())
//         .collect::<Vec<String>>().join(" ")
// }
//
// fn reverse_words(str: &str) -> String {
//     str.split(' ')
//         .into_iter()
//         .map(|word| word.chars().rev().collect::<String>())
//         .fold(String::new(), |r, c| r + c.as_str() + " ")
//         .trim_end()
//         .into()
// }
//
// fn reverse_words(str: &str) -> String {
//     let parts = str.split(" ");
//     let mut res = String::new();
//     for part in parts {
//       res = res+  &part.chars().rev().collect::<String>();
//       res.push(' ');
//     }
//     res.pop();
//     res
// }
//
// fn reverse_words(str: &str) -> String {
//     let words: Vec<_> = str.split(' ').map(|w| {
//         w.chars().rev().collect::<String>()
//     }).collect();
//     words.join(" ")
// }
