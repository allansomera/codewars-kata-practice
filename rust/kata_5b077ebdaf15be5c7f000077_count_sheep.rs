fn count_sheep(n: u32) -> String {
    let mut v = Vec::<String>::new();
    for i in 1..=n {
        v.push(i.to_string());
    }
    let s: String = v
        .into_iter()
        .map(|mut x| {
            x.push_str(" sheep...");
            x
        })
        .collect();
    s
}

fn main() {
    println!("{:?}", count_sheep(3));
}

// soln:
// fn count_sheep(n: u32) -> String {
//     (1..=n).map(|x| format!("{} sheep...", x)).collect()
// }
//
// fn count_sheep(n: u32) -> String {
//     (1..=n)
//         .map(|i| format!("{} sheep...", i))
//         .collect::<String>()
// }
//
// fn count_sheep(n: u32) -> String {
//     let mut s = String::new();
//     for i in 0..n {
//         s.push_str(&format!("{} sheep...", 1 + i));
//     }
//     s
// }
//
// fn count_sheep(n: u32) -> String {
//     (1..=n).map(|i| format!("{} sheep...", i)).collect()
// }
// // ====
// fn count_sheep(n: u32) -> String {
//     (1..=n).map(murmur).collect()
// }
//
// fn murmur(n: u32) -> String {
//     n.to_string() + " sheep..."
// }
// // ===
//
// fn count_sheep(n: u32) -> String {
//     (1..n + 1)
//         .into_iter()
//         .map(|n| format!("{} sheep...", n))
//         .collect::<Vec<String>>()
//         .join("")
// }
//
// fn count_sheep(n: u32) -> String {
//     match n {
//         0 => String::from(""),
//         _ => format!("{}{} sheep...", count_sheep(n - 1), n)
//     }
// }
// // ===
// use itertools::Itertools;
//
// fn count_sheep(n: u32) -> String {
//     (1..=n).map(|i| format!("{i} sheep...")).join("")
// }
// // ===
//
// fn count_sheep(n: u32) -> String {
//     let mut murmur = String::new();
//     for i in 1..=n { murmur += &format!("{} sheep...", i) }
//
//     murmur
// }
