fn get_age(age: &str) -> u32 {
    age.split(" ").nth(0).unwrap().parse::<u32>().unwrap()
}

fn main() {
    println!("{:?}", get_age("11 years old"));
}

// soln:
// fn get_age(age: &str) -> u32 {
//     age[..1].parse().unwrap()
// }
//
// fn get_age(age: &str) -> u32 {
//   age.chars().next().unwrap().to_digit(10).unwrap()
// }
//
// fn get_age(age: &str) -> u32 {
//     age[..1].parse::<u32>().unwrap()
// }
//
// fn get_age(age: &str) -> u32 {
//   age[..1].parse().unwrap_or(0)
// }
//
// fn get_age(age: &str) -> u32 {
//     (age.bytes().next().unwrap() - b'0') as _
// }
//
// fn get_age(age: &str) -> u32 {
//   age.chars()
//       .nth(0)
//       .expect("")
//       .to_digit(10)
//       .expect("")
// }
//
// fn get_age(age: &str) -> u32 {
//   u32::from_str_radix(&age[0..1], 10).unwrap()
// }
//
// fn get_age(age: &str) -> u32 {
//     let num: u32 = age.split_whitespace().nth(0).unwrap().parse().unwrap();
//     return num;
// }
//
// fn get_age(age: &str) -> u32 {
//     age.chars().next().unwrap() as u32 - '0' as u32
// }
//
// fn get_age(age: &str) -> u32 {
//     let age = age.split(" ").collect::<Vec<&str>>();
//     let age_int: u32 = age[0].parse().unwrap_or(0);
//     age_int
// }
