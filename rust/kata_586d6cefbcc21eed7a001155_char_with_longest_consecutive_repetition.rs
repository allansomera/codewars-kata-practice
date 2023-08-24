// extern crate regex::Regex;
use regex::regex;

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    let re = Regex::new(r"(.)\1*").unwrap();
    let Some(cap) = re.captures(s) else {return};
    println!(":?", cap);
    Option('a', 1)
}

fn main() {
    println!(":?", longest_repetition(&"abbbbb"))
}
