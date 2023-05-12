fn name_shuffler(s: &str) -> String {
    let shuf: String = String::from(s)
        .split_whitespace()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
        .to_string();
    shuf
}

fn main() {
    println!("{:?}", name_shuffler("john McLane"));
}

// soln:
// fn name_shuffler(s: &str) -> String {
//     s.rsplit(" ").collect::<Vec<&str>>().join(" ")
// }
//
// fn name_shuffler(s: &str) -> String {
//     let (f, l) = s.split_once(' ').unwrap();
//     [l, f].join(" ")
// }
//
// fn name_shuffler(s: &str) -> String {
//     s.rsplit(' ').collect::<Vec<_>>().join(" ")
// }
//
// fn name_shuffler(s: &str) -> String {
//     let arr:Vec<&str> = s.split(' ').collect();
//     format!("{} {}",arr[1],arr[0])
// }
//
// fn name_shuffler(s: &str) -> String {
//     let mut vec_name:Vec<&str> = s.split(" ").collect();
//     vec_name.reverse();
//     vec_name.join(" ")
// }
//
// fn name_shuffler(s: &str) -> String {
//     let words: Vec<& str> = s.split(" ").collect();
//     [words[1],words[0]].join(" ")
// }
//
// fn name_shuffler(s: &str) -> String {
//     let splited: Vec<&str> = s.split(" ").collect();
//     let mut result = String::from(splited[1]);
//     result.push_str(" ");
//     result.push_str(splited[0]);
//     return result;
// }
//
// // ===
// use itertools::Itertools;
//
// fn name_shuffler(s: &str) -> String {
//     s.split(" ").collect::<Vec<_>>().into_iter().rev().join(" ")
// }
// // ===
//
// fn name_shuffler(s: &str) -> String {
//     let mut rev = s.split(" ").collect::<Vec<&str>>();
//     rev.reverse();
//     rev.join(" ")
// }
//
// fn name_shuffler(s: &str) -> String {
//     let mut parts: Vec<&str> = s.split(" ").collect();
//     parts.reverse();
//     return parts.join(" ");
// }
//
// fn name_shuffler(s: &str) -> String {
//     s
//         .split_whitespace()
//         .rev()
//         .map(|x| x.to_owned())
//         .collect::<Vec<String>>()
//         .join(" ")
// }
