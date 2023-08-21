use std::collections::HashMap;

fn roman_as_num(roman: &str) -> u64 {
    if roman == "" {
        return 0;
    }
    let conversions = HashMap::from([
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ]);
    let r = vec!["I", "V", "X", "L", "C", "D", "M"];
    // let s = roman
    //     .chars()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>();

    // let vs: Vec<String> = roman.chars().map(|x| x.to_string()).collect::<Vec<_>>();
    let vs_idx = roman
        .chars()
        .map(|x| {
            r.clone()
                .into_iter()
                .position(|r| r.to_string() == x.to_string())
                .unwrap()
        })
        .collect::<Vec<_>>();
    // let find_x = vs.iter().position(|x| x == "X").unwrap_or_default();
    let mut prev: u64 = *vs_idx.first().unwrap() as u64;
    let mut vs_greater: Vec<u64> = Vec::new();
    for (i, x) in vs_idx.iter().enumerate() {
        if i == 0 {
            // prev = *x as i32;
            vs_greater.push(0);
            continue;
        }
        if *x as u64 > prev {
            vs_greater.push(1);
        } else {
            vs_greater.push(0);
        }
        prev = *x as u64;
    }
    // let ans: Vec<i32> = vs_greater
    //     .clone()
    //     .into_iter()
    //     .enumerate()
    //     .map(|(i, mut x)| {
    //         if vs_greater[i] == 1 {
    //             x = conversions[r[vs_idx[i]]] - conversions[r[vs_idx[i - 1]]];
    //             vs_greater[i - 1] = conversions[r[vs_idx[i]]] * -1;
    //         } else {
    //             x = conversions[r[vs_idx[i]]]
    //         }
    //         x
    //     })
    //     .collect::<Vec<i32>>();

    for i in 0..vs_greater.len() {
        if vs_greater[i] == 1 {
            vs_greater[i] = conversions[r[vs_idx[i]]] - conversions[r[vs_idx[i - 1]]];
            // vs_greater[i - 1] = conversions[r[vs_idx[i]]] * -1;
            vs_greater[i - 1] = 0;
        } else {
            vs_greater[i] = conversions[r[vs_idx[i]]]
        }
    }

    // for (i, &x) in vs_greater.iter().enumerate() {
    //     if x == 1 {
    //         vs_greater[i] = conversions[r[vs_idx[i]]] - conversions[r[vs_idx[i - 1]]];
    //         // vs_greater[i - 1] = conversions[r[vs_idx[i]]] * -1;
    //         vs_greater[i - 1] = 0;
    //     } else {
    //         vs_greater[i] = conversions[r[vs_idx[i]]]
    //     }
    // }

    // println!("vs: {:?}", vs);
    // println!("find_x: {:?}", find_x);
    // println!("conversions: {:?}", conversions);
    println!("r: {:?}", r);
    println!("vs_idx: {:?}", vs_idx);
    println!("vs_greater: {:?}", vs_greater);
    // println!("ans: {:?}", vs_greater.iter().sum::<u64>());
    vs_greater.iter().sum::<u64>()
    // println!("ans: {:?}", ans);
}

fn main() {
    println!("{:?}", roman_as_num("XXI"));
    println!("{:?}", roman_as_num("MCMXC"));
    println!("{:?}", roman_as_num("MMMCM"));
    println!("{:?}", roman_as_num("IX"));
}

// soln:
// use lazy_static::lazy_static;
// use std::collections::HashMap;
//
// lazy_static! {
//     static ref SYM_VAL: HashMap<char, u64> = HashMap::from([
//         ('I', 1),
//         ('V', 5),
//         ('X', 10),
//         ('L', 50),
//         ('C', 100),
//         ('D', 500),
//         ('M', 1_000)
//     ]);
// }
//
// fn roman_as_num(roman: &str) -> u64 {
//     let mut sum = 0;
//     let mut prev = 0;
//
//     for sym in roman.chars().rev() {
//         let val = SYM_VAL[&sym];
//         if val >= prev {
//             sum += val;
//         } else {
//             sum -= val;
//         }
//         prev = val;
//     }
//
//     sum
// }
//
// fn roman_as_num(roman: &str) -> u64 {
//     let mut sum = 0;
//     let mut before;
//     let mut present = 0;
//     for char in roman.chars() {
//         before = present;
//         present = match char {
//             'I' => 1,
//             'V' => 5,
//             'X' => 10,
//             'L' => 50,
//             'C' => 100,
//             'D' => 500,
//             'M' => 1000,
//             _ => unreachable!(),
//         };
//         if present > before {
//             sum += present - 2 * before;
//         } else {
//             sum += present;
//         }
//     }
//     sum
// }
//
// fn roman_as_num(roman: &str) -> u64 {
//     let mut result = 0;
//     let mut prev_value: u64 = 1001;
//
//     for c in roman.chars() {
//         let value = match c {
//             'M' => 1000,
//             'D' => 500,
//             'C' => 100,
//             'L' => 50,
//             'X' => 10,
//             'V' => 5,
//             'I' => 1,
//             _ => continue
//         };
//
//         result += value;
//
//         if value > prev_value {
//             result -= 2 * prev_value
//         }
//
//         prev_value = value
//     }
//
//     result
// }
//
// fn roman_as_num(roman: &str) -> u64 {
//     let mut out = 0;
//     let mut last = 0;
//     for c in roman.chars() {
//         let mut n = match c {
//             'I' => 1,
//             'V' => 5,
//             'X' => 10,
//             'L' => 50,
//             'C' => 100,
//             'D' => 500,
//             'M' => 1000,
//             _ => 0,
//         };
//
//         if last != 0 && last < n {
//             out -= last;
//             n -= last;
//         }
//         out += n;
//         last = n;
//     }
//
//     out
// }
//
// use std::collections::HashMap;
//
// fn roman_as_num(roman: &str) -> u64 {
//   let mut roman_values: HashMap<char, u64> = HashMap::new();
//     roman_values.insert('I', 1);
//     roman_values.insert('V', 5);
//     roman_values.insert('X', 10);
//     roman_values.insert('L', 50);
//     roman_values.insert('C', 100);
//     roman_values.insert('D', 500);
//     roman_values.insert('M', 1000);
//
//     let mut result = 0;
//     let mut prev_value = 0;
//
//     // Loop through the characters in reverse order
//     for c in roman.chars().rev() {
//         if let Some(&value) = roman_values.get(&c) {
//             if value < prev_value {
//                 result -= value;
//             } else {
//                 result += value;
//             }
//             prev_value = value;
//         }
//     }
//
//     result
// }
//
// fn roman_as_num(roman: &str) -> u64 {
//     let r_list: Vec<char> = roman.chars().collect();
//     let mut total: i64 = 0;
//     let dict = |x| match x {
//         'I' => 1,
//         'V' => 5,
//         'X' => 10,
//         'L' => 50,
//         'C' => 100,
//         'D' => 500,
//         _   => 1000
//     };
//
//
//     for x in 0..roman.len() {
//         let first = dict(r_list[x]);
//
//         if x+1 < roman.len() && first < dict(r_list[x+1]) {
//             total -= first
//         } else {
//             total += first
//         }
//     }
//
//     total as u64
// }
