fn make_upper_case(s: &str) -> String {
    let vec_s: String = String::from(s)
        .chars()
        .map(|x| x.to_uppercase().to_string())
        .collect::<Vec<_>>()
        .join("");
    vec_s
}

fn main() {
    println!("{:?}", make_upper_case("aaaaa"))
}

// soln:
// fn make_upper_case(s: &str) -> String {
//     s.to_uppercase()
// }
//
// fn make_upper_case(s: &str) -> String {
//     s.chars().flat_map(char::to_uppercase).collect()
// }
