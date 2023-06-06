fn printer_error(s: &str) -> String {
    let valid: Vec<char> = ('a'..='m').into_iter().collect::<Vec<char>>();
    let count: String = s.chars().filter(|c| !valid.contains(c)).count().to_string();
    let result = format!("{}/{}", count, s.len());
    result
}

fn main() {
    println!(
        "{:?}",
        printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz")
    );
}

// soln:
// fn printer_error(s: &str) -> String {
//     format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
// }
//
// fn printer_error(s: &str) -> String {
//     let total = s.len();
//     let bad = s.chars().filter(|&c| c < 'a' || c > 'm').count();
//     format!("{}/{}", bad, total)
// }
//
// fn printer_error(s: &str) -> String {
//     format!("{}/{}", s.bytes().map(|n| n / 110).sum::<u8>(), s.len())
// }
//
// fn printer_error(s: &str) -> String {
//     let cnt = s.matches(|c| c > 'm').count();
//     format!("{}/{}", cnt, s.len())
// }
//
// fn printer_error(s: &str) -> String {
//     format!("{}/{}",
//         s.chars().filter(|x| !"abcdefghijklm".contains(*x)).collect::<Vec<char>>().len(),
//         s.len()
//     )
// }
//
// fn printer_error(s: &str) -> String {
//     let valid_chars = 'a'..='m';
//     let good_colors = s.chars().filter(|c| valid_chars.contains(&c)).count();
//     return format!("{}/{}",s.len()-good_colors,s.len())
// }
