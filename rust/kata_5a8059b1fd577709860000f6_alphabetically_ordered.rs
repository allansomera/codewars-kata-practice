fn alphabetic(s: &str) -> bool {
    let mut a = s.chars().map(|x| x.to_string()).collect::<Vec<_>>();
    a.sort();
    if a.join("") == s.to_string() {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{:?}", alphabetic("door"));
}

// soln:
// fn alphabetic(s: &str) -> bool {
//     s.as_bytes().windows(2).all(|w| w[0] <= w[1])
// }
//
// fn alphabetic(s: &str) -> bool {
//     s.chars().zip(s.chars().skip(1)).all(|(a, b)| a <= b)
// }
//
// fn alphabetic(s: &str) -> bool {
//     let mut chars: Vec<char> = s.chars().collect();
//     chars.sort();
//     let sorted: String = chars.into_iter().collect();
//     if s == sorted{
//         return true
//     }
//
//     false
// }
//
// fn alphabetic(s: &str) -> bool {
//     for (c, p) in s.chars().zip(s.chars().skip(1)) {
//         if c > p {
//             return false;
//         }
//     }
//     return true;
// }
//
// fn alphabetic(input: &str) -> bool {
//     let mut chars: Vec<char> = input.chars().collect();
//     let original_chars = chars.clone();
//     chars.sort();
//     let sorted = original_chars == chars;
//     sorted
// }
//
// fn alphabetic(s: &str) -> bool {
//     let mut chars = s.chars().collect::<Vec<char>>();
//     chars.sort();
//     let chars: String = chars.into_iter().collect();
//     chars.eq(s)
// }
