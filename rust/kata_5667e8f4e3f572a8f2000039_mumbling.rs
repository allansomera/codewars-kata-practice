fn accum(s: &str) -> String {
    let mut v: Vec<String> = Vec::new();
    for (i, x) in s.chars().enumerate() {
        let mut temp = (0..=i)
            .map(|_| x)
            .map(|mut i| {
                i.make_ascii_lowercase();
                i.to_string()
            })
            .collect::<String>();
        if let Some(r) = temp.get_mut(0..1) {
            r.make_ascii_uppercase();
        }

        v.push(temp);
    }
    v.join("-")
}

fn main() {
    println!("{:?}", accum("ZpglnRxqenU"));
}

// soln:
// fn accum(s:&str)->String {
//     s.chars().enumerate()
//         .map(|(i,c)|c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str())
//         .collect::<Vec<String>>()
//         .join("-")
// }
//
// fn accum(s:&str)->String {
//     s.chars().enumerate()
//     .map(|(i,c)| c.to_string().to_uppercase() +
//           &(0..i).map(|_| c.to_string().to_lowercase()).collect::<String>())
//     .collect::<Vec<_>>().join("-")
// }
//
// fn accum(s:&str)->String {
//     s.chars()
//      .enumerate()
//      .map(|(i, item)| item.to_uppercase().to_string() + &item.to_lowercase().to_string().repeat(i))
//      .collect::<Vec<_>>()
//      .join("-")
// }
//
// fn accum(s:&str)->String {
//     s.chars()
//         .enumerate()
//         .map(|(i, c)| (i, c.to_string()))
//         .map(|(i, c)| c.to_ascii_uppercase() + c.to_ascii_lowercase().repeat(i).as_str())
//         .collect::<Vec<String>>()
//         .join("-")
// }
//
// fn accum(input: &str) -> String {
//     input.chars().enumerate().map(
//         |(index, ch)| {
//             let up_ch = ch.to_uppercase().to_string();
//             let low_chs = ch.to_lowercase().to_string().repeat(index);
//             up_ch + &low_chs
//         }).collect::<Vec<String>>().join("-")
// }
