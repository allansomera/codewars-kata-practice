fn trim(phrase: &str, length: usize) -> String {
    let mut s = String::new();
    if length >= phrase.len() {
        s.push_str(&phrase[0..].to_string());
    } else {
        if length <= 3 {
            if phrase.len() < 3 {
                s.push_str(&phrase[0..length].to_string());
                s.push_str("...")
            } else {
                s.push_str(&phrase[0..=length - 1].to_string());
                s.push_str("...")
            }
        } else {
            s.push_str(&phrase[0..(length - 3)].to_string());
            s.push_str("...")
        }
    }
    s
}

fn main() {
    println!("{:?}", trim("Creating kata is fun", 14));
    println!("{:?}", trim("He", 1));
    println!("{:?}", trim("Hey", 2));
    println!("{:?}", trim("Hey", 3));
    println!("{:?}", trim("Creating kata is fun", 2));
    println!("{:?}", trim("Creating kata is fun", 50));
}

// soln:
//
// fn trim(phrase: &str, mut length: usize) -> String {
//     if phrase.len() <= length {
//         return phrase.to_string();
//     }
//
//     if length > 3 {
//         length -= 3;
//     }
//
//     (&phrase[0..length]).to_string() + "..."
// }
//
// fn trim(s: &str, k: usize) -> String {
//     match s.len()<=k {
//         true => s.to_string(),
//         _ => s[0..if k>3 {k-3} else {k}].to_string()+"..."
//     }
// }
//
// fn trim(phrase: &str, length: usize) -> String {
//     if phrase.len() <= length {
//         phrase.to_owned()
//     } else {
//         phrase[0..(if length <= 3 { length } else { length - 3 })].to_owned() + "..."
//     }
// }
//
// fn trim(phrase: &str, length: usize) -> String {
//     if phrase.len() <= length {
//         phrase.to_string()
//     } else if length <= 3 {
//         format!("{}...", phrase.chars().take(length).collect::<String>())
//     } else {
//         format!("{}...", phrase.chars().take(length-3).collect::<String>())
//     }
// }
//
// fn trim(phrase: &str, length: usize) -> String {
//     if phrase.len() <= length { return phrase.into() }
//     format!("{}...", &phrase[..length - 3 * (length > 3) as usize])
// }
//
// fn trim(s: &str, l: usize) -> String {
//     match s.chars().count() {
//         n if n <= l => s.to_string(),
//         _ if l <= 3 => format!("{}...", s.chars().take(l).collect::<String>()),
//         _ => format!("{}...", s.chars().take(l - 3).collect::<String>()),
//     }
// }
//
