use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {
    let _conversions = HashMap::from([
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ]);
    let r = vec!["I", "V", "X", "L", "C", "D", "M"];
    // let s = roman
    //     .chars()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>();

    let vs: Vec<String> = roman.chars().map(|x| x.to_string()).collect::<Vec<_>>();
    let vs_idx = roman
        .chars()
        .map(|x| {
            r.clone()
                .into_iter()
                .position(|r| r.to_string() == x.to_string())
                .unwrap()
        })
        .collect::<Vec<_>>();
    // let find_x = vs.iter().position(|x| x == "X").unwrap_or_default();
    println!("vs: {:?}", vs);
    // println!("find_x: {:?}", find_x);
    // println!("conversions: {:?}", conversions);
    println!("r: {:?}", r);
    println!("vs_idx: {:?}", vs_idx);
    1
}

fn main() {
    println!("{:?}", roman_as_num("XXI"));
    println!("{:?}", roman_as_num("MDCLXVI"));
    println!("{:?}", roman_as_num("MMVIII"));
    println!("{:?}", roman_as_num("MCMXC"));
    println!("{:?}", roman_as_num("MMMCM"));
    println!("{:?}", roman_as_num("IX"));
}
