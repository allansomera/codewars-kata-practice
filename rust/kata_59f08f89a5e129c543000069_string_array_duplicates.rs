use regex::Regex;

fn dup(arry: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"(.)+").unwrap();
}

fn main() {
    println!("{:?}", dup(["abracadabra", "allottee", "assessee"]));
}
