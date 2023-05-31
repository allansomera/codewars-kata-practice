use std::collections::HashMap;

fn xo(string: &'static str) -> bool {
    let mut hm = HashMap::new();

    for c in string.chars() {
        let lc = c.to_lowercase().next().unwrap();
        hm.entry(lc).and_modify(|v| *v += 1).or_insert(1);
    }

    if hm.contains_key(&'x') {
        // contains x
        if hm.contains_key(&'o') {
            if hm[&'x'] == hm[&'o'] {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else if !hm.contains_key(&'o') {
        // doesnt contain x or o
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{:?}", xo("ooxx"));
    println!("{:?}", xo("xooxx"));
    println!("{:?}", xo("ooxXm"));
    println!("{:?}", xo("zpzpzpp"));
    println!("{:?}", xo("zzoo"));
}

// soln:
// fn xo(s: &str) -> bool {
//     let s2 = s.to_lowercase();
//     s2.matches("x").count() == s2.matches("o").count()
// }
//
// fn xo(string: &str) -> bool {
//     string.chars().filter(|&c| c == 'X' || c == 'x').count() == string.chars().filter(|&c| c == 'O' || c == 'o').count()
// }
//
// fn xo(string: &'static str) -> bool {
//   string.to_lowercase().matches("x").count() == string.to_lowercase().matches("o").count()
// }
//
// fn xo(string: &'static str) -> bool {
//     let mut xs:u32 = 0;
//     let mut os:u32 = 0;
//     for c in string.chars() {
//         match c {
//             'x' | 'X' => xs += 1,
//             'o' | 'O' => os += 1,
//             _ => ()
//         }
//     }
//     xs == os
// }
