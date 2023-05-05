fn reverse_words(words: &str) -> String {
    let mut r: Vec<String> = words
        .split(" ")
        .map(str::to_string)
        .collect::<Vec<String>>();
    r.reverse();
    let ans: String = r.join(" ");
    ans
}

fn main() {
    // reverse_words("reverse me in order")
    println!("{:?}", reverse_words("reverse me in order"));
}

// soln:
// fn reverse_words(str: &str) -> String {
//    return str.split_whitespace()
//         .rev()
//         .collect::<Vec<_>>()
//         .join(" ");
// }
//
// fn reverse_words(input: &str) -> String {
//     input.split(' ').rev().collect::<Vec<_>>().join(" ")
// }
//
// fn reverse_words(str: &str) -> String {
//     let v: Vec<&str> = str.rsplit(' ').collect();
//
//     return v.join(" ");
// }
//
// fn reverse_words(s: &str) -> String {
//     s.rsplit(" ")
//         .collect::<Vec<&str>>()
//         .join(" ")
// }
//
// fn reverse_words(str:&str) -> String {
//     str.split_whitespace().rev().collect::<Vec<_>>().join(" ")
// }
