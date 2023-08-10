fn play_pass(s: &str, n: u32) -> String {
    let ss: Vec<String> = s.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    println!("{:?}", ss);
    "test".to_string()
}

fn main() {
    println!("{:?}", play_pass("BORN IN 2015", 1));
}
