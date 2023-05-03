fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    let ans: u8 = match name {
        "Zach" => 18,
        _ => 0,
    };
    ans
}

fn main() {
    println!("{:?}", how_many_lightsabers_do_you_own("Allan"));
}

// soln:
// fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
//   if name == "Zach" { 18 } else { 0 }
// }
//
// fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
//   if name == "Zach" { return 18;}
//   0
// }
//
// fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
//   18 * (name == "Zach") as u8
// }
//
// fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
//     u8::from(name == "Zach") * 18
// }
//
// fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
//     match name
//         .as_bytes()
//         .iter()
//         .map(|&b| b as u32)
//         .sum() {
//             390 => 18,
//             _ => 0,
//     }
// }
//
// fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
//     if name.eq(&String::from("Zach")) {
//         return 18
//     }
//     0
// }
