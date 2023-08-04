fn what_century(year: &str) -> String {
    let cent: i32 = ((year.parse::<i32>().unwrap() - 1) / 100) + 1;
    // cent = (cent / 100) as i32 + 1;
    // let _post: Vec<i32> = cent
    //     .to_string()
    //     .chars()
    //     .map(|x| x.to_string().parse::<i32>().unwrap())
    //     .collect::<Vec<_>>();

    let suffix = match cent {
        12 | 13 | 11 => "th",
        _ => match cent % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", cent, suffix)

    // println!("{:?}", post);
}

fn main() {
    println!("{:?}", what_century("1999"));
    println!("{:?}", what_century("2011"));
    println!("{:?}", what_century("2154"));
    println!("{:?}", what_century("2259"));
    println!("{:?}", what_century("1234"));
    println!("{:?}", what_century("1023"));
    println!("{:?}", what_century("2000"));
    println!("{:?}", what_century("3210"));
}

// soln:
// fn what_century(year: &str) -> String {
//     let century = (year.parse::<i32>().unwrap() + 99) / 100;
//     let suffix = match (century % 10, century % 100) {
//         (_, 11) | (_, 12) | (_, 13) => "th",
//         (1, _) => "st",
//         (2, _) => "nd",
//         (3, _) => "rd",
//         _ => "th",
//     };
//     century.to_string() + suffix
// }
//
// fn what_century(year: &str) -> String {
//     let c = (year.parse::<u32>().unwrap() + 99) / 100;
//     format!(
//         "{c}{}",
//         match c % 10 {
//             _ if c >= 10 && c <= 20 => "th",
//             1 => "st",
//             2 => "nd",
//             3 => "rd",
//             _ => "th",
//         }
//     )
// }
//
// fn what_century(year: &str) -> String {
//     let year = year.parse::<usize>().unwrap();
//     let century = (year - 1) / 100 + 1;
//     let suffix = match century % 10 {
//         _ if (11..=13).contains(&century) => "th",
//         1 => "st",
//         2 => "nd",
//         3 => "rd",
//         _ => "th",
//     };
//
//     format!("{century}{suffix}")
// }
