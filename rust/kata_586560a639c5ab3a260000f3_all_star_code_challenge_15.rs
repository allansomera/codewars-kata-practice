fn rotate(s: &str) -> Vec<String> {
    let first = s.chars().next().unwrap();
    let mut vv: Vec<String> = vec![s[1..s.len()].to_string(); s.len() as usize]
        .iter()
        .map(|x| x.chars().collect::<String>())
        .collect::<Vec<_>>();
    for (idx, x) in vv.iter_mut().enumerate() {
        x.insert(idx, first);
    }

    // vv.iter_mut().for_each(|x| {
    //     x.chars().rev().collect::<String>();
    // });
    // vv.iter_mut().for_each(drop::<&mut String>);
    vv.reverse();
    println!("{:?}", vv);
    vv
}

fn main() {
    println!("{:?}", rotate("Hello"));
}
