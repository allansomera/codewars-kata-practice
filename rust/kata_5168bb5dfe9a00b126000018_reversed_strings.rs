fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

fn main() {
    println!("{:?}", solution("world"))
}

// soln:
// fn solution(phrase: &str) -> String {
//     return phrase.chars().rev().collect::<String>();
// }
//
// fn solution(phrase: &str) -> String {
//     phrase.chars().rev().collect::<String>()
// }
