fn reverse_letters(s: &str) -> String {
    let ans: String = s
        .chars()
        .filter(|x| x.is_alphabetic())
        .rev()
        .collect::<String>();
    ans
}

fn main() {
    println!("{:?}", reverse_letters("ultr53o?n"));
}

// soln:
// fn reverse_letters(s: &str) -> String {
//     s.rmatches(char::is_alphabetic).collect()
// }
//
// fn reverse_letters(s: &str) -> String {
//   s.chars().filter(|c| c.is_ascii_alphabetic()).rev().collect()
// }
