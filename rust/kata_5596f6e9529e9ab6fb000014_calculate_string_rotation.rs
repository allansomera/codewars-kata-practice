fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    let mut split: Vec<String> = first
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    // println!("{:?}", split);

    for i in 0..split.len() {
        let tmp: String = split.clone().join("");
        if tmp == second.clone().to_string() {
            return Some(i);
        }
        split.rotate_right(1);
        // println!("split: {:?}", split);
        // println!("tmp: {:?}", tmp);
        // println!("second: {:?}", second.clone().to_string())
    }
    None
}

fn main() {
    println!("{:?}", shifted_diff("eecoff", "coffee"));
}

// soln:
// fn shifted_diff(first: &str, second: &str) -> Option<usize> {
//     if first.len() > second.len() { return None }
//     second.repeat(2).find(first)
// }
//
// fn shifted_diff(first: &str, second: &str) -> Option<usize> {
//     (0..second.len()).map(|i| second[i..].to_string() + &second[..i]).position(|s| s == first)
// }
//
// fn shifted_diff(s: &str, t: &str) -> Option<usize> {
//     (0..s.len()).position(|i| s[s.len()-i..].to_string()+&s[0..s.len()-i]==t)
// }
//
// fn shifted_diff(first: &str, second: &str) -> Option<usize> {
//     if first.len()!=second.len() {return None;}
//     (String::from(second)+second).find(first)
// }
//
// fn shifted_diff(first: &str, second: &str) -> Option<usize> {
//     let mut s = second.to_string();
//     (0..=first.len()).find(|&v|{
//         if v > 0 {
//             let c = s.remove(0);
//             s.push(c);
//         }
//         if &s == first { true } else { false }
//     })
// }
