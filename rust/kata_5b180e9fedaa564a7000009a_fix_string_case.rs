fn solve(s: &str) -> String {
    let lc = s.chars().filter(|c| c.is_ascii_lowercase()).count();
    let uc = s.chars().filter(|c| c.is_ascii_uppercase()).count();
    let mut nstr = String::from(s);
    if lc > uc {
        nstr = nstr.to_ascii_lowercase();
    } else if uc > lc {
        nstr = nstr.to_ascii_uppercase();
    } else {
        nstr = nstr.to_ascii_lowercase();
    }
    nstr
}

fn main() {
    println!("{:?}", solve("CODe"));
}

// soln:
//
// fn solve(s: &str) -> String {
//     let upper_chars = s
//         .matches(|ch: char| ch.is_uppercase()) // grab only uppercase chars
//         .count();
//
//     let lower_chars = s.len() - upper_chars;
//
//     if upper_chars > lower_chars {
//         s.to_uppercase()
//     } else {
//         s.to_lowercase()
//     }
// }
//
// fn solve(s: &str) -> String {
//     let (lowers, uppers): (Vec<char>, _) = s.chars().partition(|c| c.is_lowercase());
//
//     match (lowers.len(), uppers.len()) {
//         (a, b) if a >= b => s.to_lowercase(),
//         _ => s.to_uppercase()
//     }
// }
//
// fn solve(s: &str) -> String {
//     if s.chars().filter(|x| x.is_uppercase()).count() as f32 > s.len() as f32 / 2. {
//         s.to_uppercase()
//     }
//     else{
//         s.to_lowercase()
//     }
// }
//
// fn solve(s: &str) -> String {
//     if s.chars().filter(|c| c.is_uppercase()).count() <= s.len() / 2 {
//         return s.to_lowercase();
//     }
//     s.to_uppercase()
// }
//
// fn solve(s: &str) -> String {
//     let n = s.len();
//     if s.chars().filter(|c| c.is_uppercase()).count() > n/2 {
//         s.to_uppercase()
//     } else {
//         s.to_lowercase()
//     }
// }
//
// fn solve(s: &str) -> String {
//     let lower = s.matches(|c: char| c.is_lowercase()).into_iter().count();
//     if lower >= s.chars().count() / 2 {
//         s.to_lowercase()
//     } else {
//         s.to_uppercase()
//     }.to_string()
// }
