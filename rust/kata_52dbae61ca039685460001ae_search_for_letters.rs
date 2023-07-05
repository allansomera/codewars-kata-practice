use std::collections::HashMap;

fn change(string: &str) -> String {
    let mut count = HashMap::new();
    let mut vv: Vec<i32> = vec![0; 26];
    // let alpha = b"abcdefghijklmnopqrstuvwxyz";
    // println!("{:?}", alpha);
    for c in string.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    for (k, _) in count.into_iter() {
        let lc = k.to_ascii_lowercase();
        if lc.is_ascii_lowercase() {
            // let letter = alpha[(lc as u8 - b'a') as usize];
            // println!("letter is: {}", alpha[(lc as u8 - b'a') as usize]);
            // println!("lc is: {}", lc);
            vv[(lc as u8 - b'a') as usize] = 1;
            // println!("k is: {}", k);
            // println!("index is: {}", ((lc as u8) - b'a') as usize);
            // if v >= 1 {
            //     vv[((letter - b'A') as usize) - 1] = 1;
            // }
        }
    }
    vv.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}
fn main() {
    println!("{:?}", change("b **&  bZ"));
}

// soln:
// fn change(string: &str) -> String {
//     let s = string.to_lowercase();
//     "abcdefghijklmnopqrstuvwxyz"
//       .chars()
//       .map(|c| (s.contains(c) as u32).to_string())
//       .collect()
// }
// fn change(string: &str) -> String {
//     let uc_string = string.to_uppercase();
//     (0..26).map(|l| if uc_string.contains((l as u8 + 65) as char) { '1' } else { '0' }).collect()
// }
//
// fn change(mut s: &str) -> String {
//     let mut result = vec!['0'; 26];
//
//     for c in s.to_lowercase().chars() {
//         if c.is_ascii_alphabetic() {
//             result[(c as u8 - 'a' as u8) as usize] = '1'
//         }
//     }
//
//     result.iter().collect()
// }
//
// fn change(s: &str) -> String {
//     [0;26]
//     .iter()
//     .enumerate()
//     .map(|(i,_)| {
//         if s.to_ascii_lowercase().contains(char::from_u32(i as u32+97).unwrap()) {'1'}
//         else {'0'}
//     })
//     .collect::<String>()
// }
//
// fn to_char(code: u8, offset: char) -> char {
//     (code + (offset as u8)) as char
// }
//
// fn change(string: &str) -> String {
//     let s = string.to_lowercase();
//
//     (0u8..26u8).map(|c| to_char(s.contains(to_char(c, 'a')) as u8, '0')).collect()
// }
//
// fn change(string: &str) -> String {
//     string
//         .chars()
//         .filter(char::is_ascii_alphabetic)
//         .fold(["0"; 26], |mut acc, c| {
//             let pos = c.to_ascii_lowercase() as usize - 'a' as usize;
//             acc[pos] = "1";
//             acc
//         })
//         .concat()
// }
//
// fn change(string: &str) -> String {
//     let string = string.to_lowercase();
//     (b'a'..=b'z').map(|c| if string.contains(c as char) {"1"} else {"0"}).collect()
// }
