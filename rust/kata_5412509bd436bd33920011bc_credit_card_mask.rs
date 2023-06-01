fn maskify(cc: &str) -> String {
    if cc.len() >= 4 {
        let mut mask = String::from(cc);
        let four = mask.split_off(mask.len() - 4);
        mask = mask.replace(|_| true, "#");
        mask.push_str(&four);
        mask
    } else {
        cc.to_string()
    }
}

fn main() {
    // println!("{:?}", maskify("4556364607935616"));
    println!("{:?}", maskify("1"));
}

// soln:
// fn maskify(cc: &str) -> String {
//     let mask_length = cc.len().saturating_sub(4);
//     "#".repeat(mask_length) + &cc[mask_length..]
// }
//
// fn maskify(cc: &str) -> String {
//     if cc.len() > 4{
//         "#".repeat(cc.len()-4) + &cc[cc.len() - 4..]
//     }else {
//         cc.to_string()
//     }
// }
//
// fn maskify(cc: &str) -> String {
//     match cc.len() > 4 {
//         true => "#".repeat(cc.len() - 4) + &cc[cc.len() - 4..],
//         false => cc.to_string(),
//     }
// }
//
// fn maskify(cc: &str) -> String {
//     let idx = cc.len().max(4) - 4;
//     "#".repeat(idx) + cc.get(idx..).unwrap()
// }
//
// fn maskify(code: &str) -> String {
//     let l = code.len();
//     if l <= 4 { code.to_string() }
//     else { format!("{}{}", "#".repeat(l-4), &code[l-4..]) }
// }
//
// fn maskify(cc: &str) -> String {
//     format!("{:#>1$}", &cc[cc.len().saturating_sub(4)..], cc.len())
// }
//
// fn maskify(cc: &str) -> String {
//     let index = cc.len().checked_sub(4).unwrap_or(0);
//     "#".repeat(index) + &cc[index..]
// }
//
// fn maskify(cc: &str) -> String {
//     match cc.len() {
//         0..=4 => cc.to_string(),
//         l => "#".repeat(l-4)+&cc[l-4..]
//     }
// }
//
// fn maskify(cc: &str) -> String {
//     cc.chars().enumerate().map(|(i, c)| {
//         if 4 < cc.len() - i {
//             '#'
//         } else {
//             c
//         }
//     }).collect()
// }
