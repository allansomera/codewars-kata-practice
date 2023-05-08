fn two_sort(arr: &[&str]) -> String {
    let mut x = arr.to_vec();
    x.sort();
    let first = x[0]
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("***");
    first
}

fn main() {
    println!(
        "{:?}",
        two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"])
    );
}

// soln:
// fn two_sort(arr: &[&str]) -> String {
//     arr.iter().min().unwrap().chars().map(|c| c.to_string()).collect::<Vec<_>>().join("***")
// }
//
// fn two_sort(arr: &[&str]) -> String {
//     arr.iter()
//         .min()
//         .unwrap()
//         .chars()
//         .fold(String::new(), |acc, s| format!("{}***{}", acc, s))[3..]
//         .to_string()
// }
//
// //wow i did the exact same thing, and this one got voted as 'clever'
// fn two_sort(arr: &[&str]) -> String {
//     let mut v = arr.to_vec();
//     v.sort();
//     v[0].chars().map(|c| c.to_string()).collect::<Vec<String>>().join("***")
// }
//
// fn two_sort(arr: &[&str]) -> String {
//     let mut res = arr.iter().min().expect("Array is empty.").to_string();
//     for i in (1..res.len()).rev() {
//         res.insert_str(i, "***");
//     }
//     res
// }
//
// fn two_sort(arr: &[&str]) -> String {
//     let mut ls = arr.into_iter().collect::<Vec<&&str>>();
//     ls.sort();
//     ls[0].chars()
//          .collect::<Vec<char>>()
//          .iter()
//          .map(|c| c.to_string())
//          .collect::<Vec<String>>()
//          .join("***")
// }
//
// fn two_sort(arr: &[&str]) -> String {
//     let min = arr.iter().min().unwrap().to_string();
//     min.chars().map(|c| c.to_string()).collect::<Vec<_>>().join("***")
// }
//
// fn two_sort(r: &[&str]) -> String {
//     let mut x: Vec<&str> = r.to_vec();
//     x.sort();
//     let c: Vec<&str> = x[0].split("").collect();
//     let o = &c.as_slice()[1..(c.len()-1)];
//     o.join("***").to_string()
// }
