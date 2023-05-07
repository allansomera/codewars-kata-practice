fn string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>()
}

fn main() {
    println!("{:?}", string_to_array("this is a string"));
}

// soln:
// fn string_to_array(s: &str) -> Vec<String> {
//     s.split_whitespace().map(str::to_string).collect()
// }
//
// fn string_to_array(s: &str) -> Vec<String> {
//     s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
// }
//
// fn string_to_array(s: &str) -> Vec<String> {
//     s.split(" ")
//         .map(String::from)
//         .collect()
// }
//
// fn string_to_array(s: &str) -> Vec<String> {
//   s
//     .split_whitespace()
//     .map(|w| w.to_owned())
//     .collect::<Vec<_>>()
// }
//
// fn string_to_array(s: &str) -> Vec<String> {
//     s.split_ascii_whitespace().map(String::from).collect()
// }
//
// fn string_to_array(s: &str) -> Vec<String> {
//     s.as_bytes()
//         .split(|&b| b == b' ')
//         .map(|word| unsafe { String::from_utf8_unchecked(word.to_vec()) })
//         .collect()
// }
//
// fn string_to_array(s: &str) -> Vec<String> {
//     s.split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect()
// }
