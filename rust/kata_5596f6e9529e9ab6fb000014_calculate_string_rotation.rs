fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    let mut split: Vec<String> = first
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    // println!("{:?}", split);

    for i in 0..split.len() {
        split.rotate_right(1);
        let tmp: String = split.clone().join("");
        if tmp == second.clone().to_string() {
            return Some(i);
        }
        // println!("split: {:?}", split);
        // println!("tmp: {:?}", tmp);
        // println!("second: {:?}", second.clone().to_string())
    }
    None
}

fn main() {
    println!("{:?}", shifted_diff("eecoff", "coffee"));
}
