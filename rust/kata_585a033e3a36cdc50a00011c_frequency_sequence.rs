use std::collections::HashMap;
#[allow(unreachable_patterns)]
fn freq_seq(s: &str, sep: &str) -> String {
    let mut freq = HashMap::new();

    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    // for c in freq.iter() {
    //     println!("{:?}", c);
    // }

    let str: String = s
        .chars()
        .map(|x| match x {
            x => freq[&x].to_string(),
            _ => unreachable!(),
        })
        .collect::<Vec<String>>()
        .join(sep);
    str
}

fn main() {
    println!("{:?}", freq_seq("hello world", "-"));
}

// soln:
// fn freq_seq(s: &str, sep: &str) -> String {
//     s.chars()
//         .map(|c| s.matches(c).count().to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }
//
// fn freq_seq(s: &str, sep: &str) -> String {
//     use std::collections::HashMap;
//     let mut f = HashMap::new();
//     for c in s.chars() {
//         *f.entry(c).or_insert(0_u32) += 1;
//     }
//     s.chars().map(|c| f.get(&c).unwrap_or(&0_u32).to_string()).collect::<Vec<String>>().join(sep)
// }
//
// use std::collections::HashMap;
//
// fn freq_seq(s: &str, sep: &str) -> String {
//     let mut d: HashMap<char, usize> = HashMap::new();
//     for c in s.chars(){
//         *d.entry(c).or_default() += 1;
//     }
//     s.chars().map(|x| d[&x].to_string()).collect::<Vec<_>>().join(sep)
// }
//
// fn freq_seq(s: &str, sep: &str) -> String {
//     s.chars().map(|c| s.matches(c).count().to_string()).collect::<Vec<_>>().join(sep)
// }
//
// use std::collections::HashMap;
//
// fn freq_seq(s: &str, sep: &str) -> String {
//     let mut chars_count = HashMap::new();
//     for ch in s.chars() {
//         *chars_count.entry(ch).or_insert(0) += 1;
//     }
//     let new_chars: Vec<_> = s.chars()
//         .map(|ch| chars_count.get(&ch).unwrap().to_string())
//         .collect();
//     new_chars.join(sep)
// }
