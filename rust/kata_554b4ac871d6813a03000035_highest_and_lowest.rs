fn high_and_low(numbers: &str) -> String {
    let mut v: Vec<i32> = numbers
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect::<Vec<i32>>();
    v.sort();

    let vs = v
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut vv: Vec<&str> = match (vs.split_first(), vs.split_last()) {
        (Some((first, _)), Some((last, _))) => vec![first, last],
        _ => Vec::new(),
    };
    vv.sort_unstable_by(|a, b| b.cmp(a));
    vv.join(" ")
}

fn main() {
    println!("{:?}", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

// soln:
// fn high_and_low(numbers: &str) -> String {
//     use std::cmp;
//     let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));
//
//     let answer = numbers
//         .split_whitespace()
//         .map(|x| x.parse::<i32>().unwrap())
//         .fold((i32::min_value(), i32::max_value()), f);
//     format!("{} {}", answer.0, answer.1)
// }
//
// fn high_and_low(numbers: &str) -> String {
//   let as_ints: Vec<i32> = numbers.split(" ").map(|x| x.parse().unwrap()).collect();
//   format!("{} {}", as_ints.iter().max().unwrap(), as_ints.iter().min().unwrap())
// }
//
// fn high_and_low(numbers: &str) -> String {
//   let n = String::from(numbers);
// 	let vect :Vec<i32>= n.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
// 	let low = vect.iter().min().unwrap();
// 	let high = vect.iter().max().unwrap();
//   format!("{} {}", high, low)
// }
//
// fn high_and_low(numbers: &str) -> String {
//     let result: Vec<i32> = numbers
//         .split_whitespace()
//         .map(|x| x.parse::<i32>().unwrap())
//         .collect();
//
//     format!(
//         "{} {}",
//         result.iter().max().unwrap(),
//         result.iter().min().unwrap()
//     )
// }
