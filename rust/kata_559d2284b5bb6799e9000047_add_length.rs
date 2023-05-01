fn add_length(s: &str) -> Vec<String> {
    let vec_s: Vec<String> = s
        .split(" ")
        .map(str::to_string)
        .map(|mut x| {
            x.push_str(&format!(" {}", x.len().to_string()));
            x
        })
        .collect();
    vec_s
}

fn main() {
    println!("{:?}", add_length("apple ban"));
}

// soln:
// fn add_length(s: &str) -> Vec<String> {
//    s.split_whitespace().map(|x| format!("{} {}", x, x.len())).collect()
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     s.split_whitespace().map(|w| format!("{} {}", w, w.len())).collect()
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     let mut words: Vec<String> = Vec::new();
//
//     for i in s.split_whitespace() {
//        words.push(format!("{} {}", i, i.len()).to_string())
//     }
//     words
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     s
//         .split_whitespace()
//         .map(|v| format!("{} {}", v, v.len()))
//         .collect()
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     s
//     .split_whitespace()
//     .into_iter()
//     .map(|x| format!("{} {}",x,x.len()))
//     .collect()
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     s.split(' ')
//         .map(|word| format!("{word} {}", word.chars().count()))
//         .collect()
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     s.split_whitespace()
//         .map(|s_| format!("{} {}", s_, s_.len()))
//         .collect::<Vec<_>>()
// }
//
// fn add_length(s: &str) -> Vec<String> {
//     s.split(" ").map(|a| format!("{} {}", a, a.len()).to_string()).collect()
// }
//
