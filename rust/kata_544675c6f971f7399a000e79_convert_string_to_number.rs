fn string_to_number(s: &str) -> i32 {
    let n: i32 = s.parse::<i32>().unwrap();
    n
}

fn main() {
    println!("{:?}", string_to_number("-7"));
}

// soln:
// fn string_to_number(s: &str) -> i32 {
//   s.parse().unwrap()
// }
//
// fn string_to_number(s: &str) -> i32 {
//   s.parse::<i32>().unwrap_or(0)
// }
//
// fn string_to_number(s: &str) -> i32
// {
//   i32::from_str_radix(s, 10).unwrap_or(0)
// }
//
// fn string_to_number(s: &str) -> i32 {
//   s.trim().parse::<i32>().unwrap()
// }
