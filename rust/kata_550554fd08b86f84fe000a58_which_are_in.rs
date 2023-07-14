fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut vs: Vec<String> = Vec::new();

    for x in arr_a.into_iter() {
        println!("{}", x);
        for y in arr_b.into_iter() {
            if y.contains(x) {
                if !vs.contains(&x.to_string()) {
                    vs.push(x.clone().to_owned());
                }
            }
        }
    }

    vs.sort_unstable();
    vs
}

fn main() {
    println!(
        "{:?}",
        in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"]
        )
    );
}

// soln:
// use itertools::Itertools;
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     arr_a
//       .iter()
//       .map(|x| x.to_string())
//       .filter(|x| { arr_b.into_iter().any(|y| y.contains(x.as_str()))} )
//       .unique()
//       .sorted()
//       .collect()
// }
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     let mut result: Vec<String> = arr_a.iter()
//         .filter(|&e| arr_b.iter().any(|&t| t.contains(e)))
//         .map(|s| s.to_string())
//         .collect();
//
//     result.sort_unstable();
//     result.dedup();
//     result
// }
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     let mut result = Vec::new();
//     for i in 0..arr_a.len(){
//         for j in 0..arr_b.len(){
//             if arr_b[j].contains(arr_a[i]) {
//                 result.push(arr_a[i].to_string());
//             }
//         }
//     }
//     result.sort();
//     result.dedup();
//     result
// }
//
// use std::collections::BTreeSet;
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     Vec::from_iter(
//         BTreeSet::from_iter(
//             arr_a
//                 .iter()
//                 .filter(|&e| arr_b.iter().any(|&t| t.contains(e)))
//                 .map(|s| s.to_string()),
//         )
//         .into_iter(),
//     )
// }
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     let mut result: Vec<String> = arr_a
//         .iter()
//         .filter(|&&str1| arr_b.iter().any(|&str2| str2.contains(&str1)))
//         .map(|&s| String::from(s))
//         .collect();
//
//     result.sort();
//     result.dedup();
//
//     result
// }
//
// use std::collections::HashSet;
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     let mut result = arr_a
//         .iter()
//         .cloned()
//         .filter(|&a| match arr_b.iter().find(|&&b| b.contains(a)) {
//             Some(_) => true,
//             None => false,
//         })
//         .map(String::from)
//         .collect::<HashSet<_>>()
//         .into_iter()
//         .collect::<Vec<_>>();
//     result.sort();
//     result
// }
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     let mut a = arr_a.to_vec();
//     a.sort();
//     a.dedup();
//     a.into_iter().filter(|&s|{ arr_b.iter().any(|x|x.contains(s)) }).map(|s|s.to_string()).collect()
// }
//
// use itertools::Itertools;
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     arr_a
//     .iter()
//     .unique()
//     .filter(|x| {arr_b.iter().any(|y| y.contains(**x))})
//     .map(|x| x.to_string())
//     .sorted()
//     .collect::<Vec<String>>()
// }
//
// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//
//     let mut  arr:Vec<String> = arr_a.iter().filter(|&v|arr_b.iter().any(|e|e.contains(v)))
//     .map(|r|r.to_string())
//     .collect();
//     arr.sort();
//     arr.dedup();
//     arr
//
// }
