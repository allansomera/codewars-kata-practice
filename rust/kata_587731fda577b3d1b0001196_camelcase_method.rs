fn camel_case(str: &str) -> String {
    // let s: Vec<String> = str
    let s: String = str
        .split_whitespace()
        .into_iter()
        // .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
        // .map(|x| x[..1].to_string().to_uppercase() + &x[1..x.len()].to_string())
        .map(|x| x[..1].to_uppercase() + &x[1..x.len()])
        .collect::<Vec<_>>()
        .join("");
    // println!("{:?}", s);
    s
}

fn main() {
    println!("{:?}", camel_case("test case"));
}

// soln:
// fn camel_case(str: &str) -> String {
//     str.split_whitespace()
//         .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
//         .collect()
// }
//
// fn camel_case(str: &str) -> String {
//     str.split_whitespace()
//         .collect::<Vec<_>>()
//         .iter()
//         .map(|&s| [&s[..1].to_uppercase(), &s[1..]].join(""))
//         .collect::<Vec<_>>()
//         .join("")
// }
//
// fn camel_case(str: &str) -> String {
//     str.split_whitespace().map(|x| {
//         let (head, tail) = x.split_at(1);
//         head.to_uppercase() + tail
//     }).collect::<String>()
// }
//
// fn camel_case(str: &str) -> String {
//     str.split_whitespace().map(str::to_lowercase).map(|mut s| {
//         s.get_mut(..1).map(str::make_ascii_uppercase);
//         s
//     }).collect::<Vec<String>>().join("")
// }
//
// fn camel_case(str: &str) -> String {
//     str.split_ascii_whitespace()
//         .map(|s| format!("{}{}", &s[0..1].to_ascii_uppercase(), &s[1..]))
//         .collect::<Vec<_>>()
//         .join("")
// }
//
// fn camel_case(str: &str) -> String {
//     str
//     .split_whitespace()
//     .map(|s| s
//         .chars()
//         .take(1)
//         .map(|c| c.to_ascii_uppercase())
//         .chain(s.chars().skip(1)))
//     .flatten()
//     .collect()
// }
