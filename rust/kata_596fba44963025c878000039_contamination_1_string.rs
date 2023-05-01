fn contamination(text: &str, character: &str) -> String {
    if text.len() == 0 {
        return "".to_string();
    }
    let s: String = String::from(text)
        .chars()
        .map(|_x| character.chars().nth(0).unwrap())
        .collect();
    s
}
// explanation:
// .chars() // turns into chars iteratator
// .map(
//     |_x| character.chars().nth(0).unwrap() // since character is type &str, need to change into char
//                                            // by using character.chars().nth(0).unwrap()
//                                            // since the char is length 1, you choose the nth(0)
//     )
//     .collect() // collect the iterator and transform back to String

fn main() {
    println!("{:?}", contamination("abccccc", "z"));
}

// soln:
// fn contamination(text: &str, character: &str) -> String {
//     character.repeat(text.len())
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     text.chars().map(|c| character).collect()
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     let num = text.len();
//     if text == "" || character == "" {
//         return "".to_string();
//     }
//     (0..num).map(|_x| format!("{}", character)).collect()
// }
//
// fn contamination(a: &str, b: &str) -> String {
//     b.repeat(a.chars().count())
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     let mut ret = String::new();
//     for c in text.chars() {
//         ret.push_str(character);
//     }
//     ret
// }
//
// fn contamination(text: &str, character: &str) -> String {
//
//     if text=="" {
//         return text.to_string()
//     }
//
//     text.chars().map(|_| character ).collect()
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     text.bytes().map(|_| character).collect()
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     if text.is_empty() || character.is_empty() {
//         return "".to_string()
//     }
//     else {
//         return (1..=text.len()).map(|x| format!("{character}")).collect();
//     }
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     if text.is_empty() || character.is_empty() {
//         String::new()
//     } else {
//         (0..text.len()).map(|_| character).collect::<String>()
//     }
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     let mut infected = String::new();
//     for i in 0..text.len() {
//         infected.push_str(character);
//     }
//     infected
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     let n = text.len();
//     std::iter::repeat(character).take(n).collect::<String>()
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     return text.chars().map(|x|
//         match x {
//             _ => character
//         }
//     ).collect();
// }
//
// fn contamination(text: &str, character: &str) -> String {
//     let mut res = String::new();
//     for _ in 0..text.chars().count(){
//         res += character;
//     }
//     return res.to_string();
// }
