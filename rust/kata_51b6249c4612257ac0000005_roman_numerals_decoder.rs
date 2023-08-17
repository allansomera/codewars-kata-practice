use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {
    let conversions = HashMap::from([
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ]);

    let vs: Vec<String> = roman.chars().map(|x| x.to_string()).collect::<Vec<_>>();
    let find_x = vs.iter().position(|x| x == "X").unwrap();
    println!("{:?}", vs);
    println!("{:?}", find_x);
    println!("{:?}", conversions);
    1
}

fn main() {
    println!("{:?}", roman_as_num("XXI"))
}
