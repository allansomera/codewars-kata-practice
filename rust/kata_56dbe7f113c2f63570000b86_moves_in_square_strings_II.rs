fn rot(s: &str) -> String {
    let vs: Vec<String> = s.split("\n").map(str::string).collect::<Vec<String>>();
    println!("");

    "test".to_string()
}
fn selfie_and_rot(s: &str) -> String {
    // your code

    "test".to_string()
}

// first parameter: dots have to be replaced by function of one variable
fn oper(f: fn(String) -> String, s: &str) -> String {
    // your code
    "test".to_string()
}

fn main() {
    let s = "abcd\nefgh\nijkl\nmnop";
    println!("{:?}", oper(rot, x));
}
