fn switcheroo(s: &str) -> String {
    let ans: String = String::from(s)
        .chars()
        .map(|x| match x {
            'a' => 'b',
            'b' => 'a',
            _ => 'c',
        })
        .collect::<String>();
    ans
}

fn main() {
    println!("{:?}", switcheroo("aabacbaa"));
}

// soln:
// fn switcheroo(s: &str) -> String {
//     s.chars().map(|e|
//         match e{
//             'a' => 'b',
//             'b' => 'a',
//             _ => e,
//     }).collect()
// }
//
// fn switcheroo(s: &str) -> String {
//     s.replace("a", "X").replace("b", "a").replace("X", "b")
// }
//
// fn switcheroo(s: &str) -> String {
//     s.chars()
//         .map(|c| match c {
//             'a' => 'b',
//             'b' => 'a',
//             c => c
//         })
//         .collect::<String>()
// }
//
// fn switcheroo(s: &str) -> String {
//     String::from_utf8(
//         s.bytes().map(|c| match c {
//             b'a' => b'b',
//             b'b' => b'a',
//             c => c
//         }).collect()
//     ).unwrap()
// }
