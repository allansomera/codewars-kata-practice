fn roman_as_num(roman: &str) -> u64 {
    let vs: Vec<String> = roman.chars().map(|x| x.to_string()).collect::<Vec<_>>();
    let find_x = vs.iter().position(|x| x == "X").unwrap();
    println!("{:?}", vs);
    println!("{:?}", find_x);
    1
}

fn main() {
    println!("{:?}", roman_as_num("XXI"))
}
