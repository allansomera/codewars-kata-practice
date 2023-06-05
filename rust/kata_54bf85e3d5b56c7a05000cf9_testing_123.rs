fn number(lines: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = (1..=lines.len()).map(|x| x.to_string()).collect();
    for (i, x) in lines.iter().enumerate() {
        v[i].push_str(&format!(": {}", x));
    }
    v
}

fn main() {
    println!("{:?}", number(&["a", "b", "c"]));
}

// soln:
// fn number(lines: &[&str]) -> Vec<String> {
//     lines.iter().enumerate().map(|x| format!("{}: {}", x.0 + 1, x.1)).collect()
// }
//
// fn number(lines: &[&str]) -> Vec<String> {
//     lines.iter()
//         .enumerate()
//         .map(|(x, y)| format!("{}: {y}", x + 1))
//         .collect()
// }
//
// fn number(lines: &[&str]) -> Vec<String> {
//     lines.iter().zip(1..).map(|(x, i)| format!("{i}: {x}")).collect()
// }
//
// fn number(lines: &[&str]) -> Vec<String> {
//     lines.iter().enumerate().map(|(i, s)| format!("{}: {}", i + 1, s)).collect()
// }
//
// fn number(lines: &[&str]) -> Vec<String> {
//     lines
//         .iter()
//         .zip(1..)
//         .map(|(val, index)| format!("{index}: {val}"))
//         .collect::<Vec<String>>()
// }
//
// fn number(lines: &[&str]) -> Vec<String> {
//     let mut res: Vec<String> = vec![];
//     for i in 0..lines.len() {
//         res.push(format!("{}: {}", i+1, lines[i]))
//     }
//     return res;
// }
