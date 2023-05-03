fn boolean_to_string(b: bool) -> String {
    b.to_string()
}

fn main() {
    println!("{:?}", boolean_to_string(false))
}

// soln:
// fn boolean_to_string(b: bool) -> String {
//     format!("{}", b)
// }
//
// fn boolean_to_string(b: bool) -> String {
//     String::from(if b { "true" } else { "false" })
// }
//
// fn boolean_to_string(b: bool) -> String {
//      ["false", "true"][b as usize].to_string()
// }
