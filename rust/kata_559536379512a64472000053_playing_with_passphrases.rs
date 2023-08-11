fn play_pass(s: &str, n: u32) -> String {
    let ss: Vec<String> = s
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect();
    println!("{:?}", ss);
    "test".to_string()
}

fn main() {
    println!("{:?}", play_pass("BORN IN 2015", 1));
}
