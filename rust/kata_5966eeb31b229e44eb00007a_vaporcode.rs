fn vaporcode(s: &str) -> String {
    let vw: Vec<String> = s
        .chars()
        .filter(|x| !x.is_whitespace())
        .map(|x| x.to_string())
        .map(|x| x.to_uppercase())
        .collect::<Vec<String>>();
    println!("{:?}", vw);
    vw.join("  ")
}
fn main() {
    println!("{:?}", vaporcode("testing"));
}

// soln:
// fn vaporcode(s: &str) -> String {
//   s.replace(" ", "").to_ascii_uppercase().chars().map(|x| x.to_string()).collect::<Vec<_>>().join("  ")
// }
//
// fn vaporcode(s: &str) -> String {
//     s.replace(" ", "")
//         .chars()
//         .map(|c| c.to_ascii_uppercase().to_string())
//         .collect::<Vec<String>>()
//         .join("  ")
// }
//
// fn vaporcode(s: &str) -> String {
//     s.to_uppercase().chars().map(|c| c.to_string()).filter(|s| s!=" ").collect::<Vec<_>>().join("  ")
// }
//
// fn vaporcode(s: &str) -> String {
//     s.chars()
//         .filter(|c| *c != ' ')
//         .map(|c| format!("{}  ", c.to_ascii_uppercase()))
//         .collect::<Vec<String>>()
//         .join("")
//         .trim()
//         .into()
// }
//
// fn vaporcode(s: &str) -> String {
//     s.chars().map(|c| if c != ' '{format!("{}  ", c.to_uppercase())} else{String::new()}).collect::<String>().trim().to_string()
// }
//
// use itertools::Itertools;
// fn vaporcode(s: &str) -> String {
//     s.split_whitespace().collect::<String>()
//         .to_uppercase()
//         .chars()
//         .join("  ")
// }
