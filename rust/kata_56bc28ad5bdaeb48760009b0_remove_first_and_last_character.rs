pub fn remove_char(s: &str) -> String {
    // let slice = &s[1..s.len() - 1];
    String::from(&s[1..s.len() - 1])
}

pub fn main() {
    println!("{}", remove_chars(&String::from("hello")));
}

// soln:
// pub fn remove_char(s: &str) -> String {
//     s[1..s.len() - 1].to_string()
// }
//
//
// pub fn remove_char(s: &str) -> String {
//     // using slice syntax is inaccurate because that is based on the false assumption that
//     // one char is always one code point.
//     // See https://doc.rust-lang.org/std/primitive.str.html#method.len
//     s.chars().skip(1).take(s.chars().count() - 2).collect()
// }
//
// pub fn remove_char(s: &str) -> String {
//   s[1..s.len()-1].into()
// }
