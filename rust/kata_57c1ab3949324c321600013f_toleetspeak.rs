fn to_leet_speak(s: &str) -> String {
    s.clone()
        .chars()
        .map(|x| match x {
            'A' => '@',
            'B' => '8',
            'C' => '(',
            'D' => 'D',
            'E' => '3',
            'F' => 'F',
            'G' => '6',
            'H' => '#',
            'I' => '!',
            'J' => 'J',
            'K' => 'K',
            'L' => '1',
            'M' => 'M',
            'N' => 'N',
            'O' => '0',
            'P' => 'P',
            'Q' => 'Q',
            'R' => 'R',
            'S' => '$',
            'T' => '7',
            'U' => 'U',
            'V' => 'V',
            'W' => 'W',
            'X' => 'X',
            'Y' => 'Y',
            'Z' => '2',
            _ => x,
        })
        .collect::<String>()
}

fn main() {
    println!("{:?}", to_leet_speak("LEET"));
}

// soln:
// use std::collections::HashMap;
//
// fn to_leet_speak(s: &str) -> String {
//     let map: HashMap<_, _> = vec![
//         ('A', '@'),
//         ('B', '8'),
//         ('C', '('),
//         ('D', 'D'),
//         ('E', '3'),
//         ('F', 'F'),
//         ('G', '6'),
//         ('H', '#'),
//         ('I', '!'),
//         ('J', 'J'),
//         ('K', 'K'),
//         ('L', '1'),
//         ('M', 'M'),
//         ('N', 'N'),
//         ('O', '0'),
//         ('P', 'P'),
//         ('Q', 'Q'),
//         ('R', 'R'),
//         ('S', '$'),
//         ('T', '7'),
//         ('U', 'U'),
//         ('V', 'V'),
//         ('W', 'W'),
//         ('X', 'X'),
//         ('Y', 'Y'),
//         ('Z', '2'),
//         (' ', ' '),
//     ].into_iter()
//         .collect();
//
//     s.chars().flat_map(|c| { map.get(&c) }).cloned().collect()
// }
//
// fn to_leet_speak(s: &str) -> String {
//     s.bytes().map(|b| {
//         (if b.is_ascii_uppercase() {
//             b"@8(D3F6#!JK1MN0PQR$7UVWXY2"[(b - b'A') as usize]
//         } else {
//             b
//         }) as char
//     }).collect()
// }
//
// fn to_leet_speak(s: &str) -> String {
//     const tab: &str = "@8(D3F6#!JK1MN0PQR$7UVWXY2";
//     s.chars().map(|c| if c >= 'A' && c <= 'Z' { tab.chars().nth(c as usize - 'A' as usize).unwrap() } else { c }).collect()
// }
//
// fn to_leet_speak(s: &str) -> String {
//     let alphabet = [
//         ['A', '@'],
//         ['B', '8'],
//         ['C', '('],
//         ['E', '3'],
//         ['G', '6'],
//         ['H', '#'],
//         ['I', '!'],
//         ['L', '1'],
//         ['O', '0'],
//         ['S', '$'],
//         ['T', '7'],
//         ['Z', '2'],
//     ];
//     s.chars()
//         .map(|l| {
//             alphabet
//             .iter()
//             .find(|&&x| x[0] == l)
//             .or(std::option::Option::Some(&[l,l]))
//             .unwrap()[1]
//         })
//         .collect()
// }
