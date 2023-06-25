fn solve(s: &str) -> String {
    let mut r: String = s
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("");
    println!("{:?}", r);
    let mut ws: Vec<usize> = Vec::new();
    for (i, x) in s.chars().enumerate() {
        if x.is_whitespace() {
            ws.push(i);
        }
    }
    for i in ws.into_iter() {
        r.insert(i, ' ');
    }
    r
}

fn main() {
    println!("{:?}", solve("codewars"));
    println!("{:?}", solve("this is a test"));
    println!("{:?}", solve("your code rocks"));
}

// soln:
// fn solve(s: &str) -> String {
//     let mut ss = s.chars().filter(|x| !x.is_ascii_whitespace()).rev().collect::<String>();
//     for (idx, ch) in s.chars().enumerate() {
//         if ch.is_ascii_whitespace() {
//             ss.insert(idx, ' ');
//         }
//     }
//     ss
// }
//
// fn solve(s: &str) -> String {
//     let mut stack = s
//         .chars()
//         .filter(char::is_ascii_alphabetic)
//         .collect::<Vec<_>>();
//
//     s.chars().fold(String::new(), |mut result, c| {
//         match c.is_ascii_alphabetic() {
//             true => result.push(stack.pop().unwrap()),
//             false => result.push(c),
//         }
//
//         result
//     })
// }
//
// fn solve(s: &str) -> String {
//     let mut reversed = s.chars().filter(|c| c != &' ').rev();
//     s.chars().map(|c| match c {
//         ' ' => ' ',
//         _ => reversed.next().unwrap()
//     }).collect()
// }
//
// fn solve(s: &str) -> String {
//     let mut rev: Vec<_> = s.replace(' ', "").chars().collect();
//     rev.reverse();
//     let mut iter = rev.into_iter();
//     s.chars().map(|ch| if ch == ' ' { ' ' } else { iter.next().unwrap() }).collect()
// }
//
// fn solve(s: &str) -> String {
//     let mut res: String = s.chars().filter(|&c| c != ' ').rev().collect();
//     for (i, c) in s.chars().enumerate() {
//         if c == ' ' {
//             res.insert(i, c);
//         }
//     }
//     res
// }
