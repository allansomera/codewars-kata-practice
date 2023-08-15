fn roman_as_num(roman: &str) -> u64 {
    let vs: Vec<String> = roman.chars().map(|x| x.to_string()).collect::<Vec<_>>();
    println!("{:?}", vs);
    1
}

fn main() {
    println!("{:?}", roman_as_num("XXI"))
}
