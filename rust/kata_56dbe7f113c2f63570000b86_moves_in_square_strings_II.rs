fn rot(s: &str) -> String {
    let vs: Vec<String> = s
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .collect::<Vec<String>>()
        .into_iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<String>>();
    vs.join("\n")
}
fn selfie_and_rot(s: &str) -> String {
    let r: Vec<String> = rot(s)
        .split("\n")
        .map(|x| x.to_string())
        .map(|x| format!("\n{}{}", ".".repeat(x.len()), x))
        .collect::<Vec<String>>();

    let new_s: Vec<String> = s
        .split("\n")
        .map(|x| x.to_string())
        .map(|x| format!("{}{}", x, ".".repeat(x.len())))
        .collect::<Vec<String>>();

    format!("{}{}", new_s.join("\n"), r.join(""))
    // new_s.join("...\n")
}

// first parameter: dots have to be replaced by function of one variable
fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}

fn main() {
    let s = "abcd\nefgh\nijkl\nmnop";
    // println!("{:?}", oper(rot, s));
    println!("{:?}", oper(selfie_and_rot, s));
}

// soln:
// //#![feature(repeat_str)] <--since repeat_str is not stable on 1.10 used by codewars we have to do it wit iter::repeat
// fn repeat(s: &str, n: usize) -> String {
//     std::iter::repeat(s).take(n).collect()
// }
// fn rot(s: &str) -> String {
//     s.chars().rev().collect()
// }
// fn selfie_and_rot(s: &str) -> String {
//     s.split('\n').map(|s| s.to_string() + &repeat(".", s.chars().count())).collect::<Vec<String>>().join("\n")
//     + "\n" +
//     &rot(s).split('\n').map(|s| repeat(".", s.chars().count()) + s).collect::<Vec<String>>().join("\n")
// }
// fn oper(oper: fn(&str) -> String, s: &str) -> String {
//     oper(s)
// }
//
// fn repeat(s: &str, n: usize) -> String {
//     (0..n).map(|_| s).collect::<String>()
// }
//
// fn rot(s: &str) -> String {
//     s.chars().rev().collect()
// }
//
// fn selfie_and_rot(s: &str) -> String {
//     s.lines()
//         .map(|line| format!("{}{}", line,  repeat(".", line.len())))
//         .chain(
//             s.lines()
//                 .map(|line| format!("{}{}", repeat(".", line.len()), rot(line)))
//                 .rev()
//         )
//         .collect::<Vec<_>>()
//         .join("\n")
// }
//
// fn oper(func: fn(&str) -> String, s: &str) -> String {
//     func(s)
// }
//
// fn rot(s: &str) -> String {
//     s.chars().rev().collect::<String>()
// }
// fn rep(s: &str, n: usize) -> String {
//     std::iter::repeat(s).take(n).collect::<String>()
// }
// fn selfie_and_rot(s: &str) -> String {
//     let newstr1 = s.split('\n')
//       .collect::<Vec<&str>>()
//       .iter().map(|u| [u.to_string(), rep(".", u.len())].join("") ).collect::<Vec<String>>()
//       .join("\n");
//     let newstr2 = rot(s).split('\n')
//       .collect::<Vec<&str>>()
//       .iter().map(|u| [rep(".", u.len()), u.to_string()].join("") ).collect::<Vec<String>>()
//       .join("\n");
//     [newstr1, "\n".to_string(), newstr2].join("")
//
// }
//
// fn oper(f: fn(&str) -> String, s: &str) -> String {
//     f(s)
// }
