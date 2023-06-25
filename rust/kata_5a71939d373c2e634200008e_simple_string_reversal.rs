fn solve(s: &str) -> String {
    let r: Vec<String> = s
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>();
    r.join(" ")
}

fn main() {
    println!("{:?}", solve("codewars"));
    println!("{:?}", solve("this is a test"));
}
