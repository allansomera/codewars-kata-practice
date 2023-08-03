fn expanded_form(n: u64) -> String {
    n.to_string()
        .chars()
        .rev()
        .enumerate()
        .map(|(i, x)| x.to_string().parse::<u64>().unwrap() * 10u64.pow(i as u32))
        .collect::<Vec<u64>>()
        .into_iter()
        .filter(|&x| x != 0)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" + ")
}

fn main() {
    println!("{:?}", expanded_form(12345));
}

// soln:
// fn expanded_form(n: u64) -> String {
//     n.to_string()
//         .chars()
//         .rev()
//         .zip(0..)
//         .filter(|&(c, _)| c != '0')
//         .map(|(c, p)| format!("{}{}", c, "0".repeat(p)))
//         .collect::<Vec<_>>()
//         .into_iter()
//         .rev()
//         .collect::<Vec<_>>()
//         .join(" + ")
// }
//
// fn expanded_form(n: u64) -> String {
//     n.to_string()
//         .chars()
//         .enumerate()
//         .filter(|(_, c)| *c != '0')
//         .map(|(i, c)| {
//             ((c as usize - 48) * 10usize.pow(n.to_string().len() as u32 - 1 - i as u32)).to_string()
//         })
//         .collect::<Vec<String>>()
//         .join(" + ")
// }
//
// fn expanded_form(n: u64) -> String {
//
//     let size = |x| {(n.to_string().len() - (x) - 1) as u32};
//
//     let f = |i, ch:char| ((ch.to_digit(10).unwrap() as u64) * 10_u64.pow(size(i))).to_string();
//
//     n.to_string()
//      .chars()
//      .enumerate()
//      .filter(|(_,ch)| *ch != '0')
//      .map(|(i,ch)| f(i,ch)).collect::<Vec<String>>().join(" + ")
//
// }
//
// fn expanded_form(n: u64) -> String {
//     let mut mag:u64 = 10u64.pow(n.to_string().chars().count() as u32);
//     n.to_string().chars()
//         .map(|c| {mag/=10; c.to_digit(10).unwrap() as u64 * mag})
//         .map(|u| format!("{}", u))
//         .filter(|s| s != "0")
//         .collect::<Vec<String>>()
//         .join(" + ")
// }
//
// fn expanded_form(n: u64) -> String {
//     n.to_string().bytes().rev().enumerate().rev().filter(|(_, x)| *x != '0' as u8).map(|(id, x)| {
//         format!("{:0<width$}", x as char, width = id + 1)
//     }).collect::<Vec<String>>().join(" + ")
// }
