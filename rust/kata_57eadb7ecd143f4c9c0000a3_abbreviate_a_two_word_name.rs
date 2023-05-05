fn abbrev_name(name: &str) -> String {
    let vec_n: String = String::from(name)
        .split(" ")
        .map(str::to_string)
        .map(|x| x.chars().nth(0).unwrap())
        .map(|x| x.to_uppercase().to_string())
        .collect::<Vec<String>>()
        .join(".");
    vec_n
}

fn main() {
    println!("{:?}", abbrev_name("bruce wayne"));
}

// soln:
// fn abbrev_name(name: &str) -> String {
//   name.split(' ')
//       .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
//       .collect::<Vec<_>>()
//       .join(".")
// }
//
// fn abbrev_name(name: &str) -> String {
//     let mut names = name.split(" ");
//     let first = names.next().unwrap();
//     let second = names.next().unwrap();
//     return first[0..1].to_uppercase().to_string() + "." + &second[0..1].to_uppercase();
// }
//
// fn abbrev_name(name: &str) -> String {
//     name
//         .split(" ")
//         .map(|word| word[0..1].to_uppercase())
//         .collect::<Vec<_>>()
//         .join(".")
// }
//
// fn abbrev_name(name: &str) -> String {
//     name.split_whitespace().take(2).map(|s| s.chars().nth(0).unwrap().to_uppercase().to_string()).collect::<Vec<_>>().join(".")
// }
//
// fn abbrev_name(name: &str) -> String {
//     let split_name = name.split_whitespace().collect::<Vec<&str>>();
//     let first = &split_name[0][0..1];
//     let last = &split_name[1][0..1];
//     format!("{}.{}", first.to_uppercase(), last.to_uppercase())
// }
//
// fn abbrev_name(name: &str) -> String {
//     let snd = name.find(" ").unwrap()+1;
//     name[0..1].to_uppercase() + "." + &name[snd..snd+1].to_uppercase()
// }
//
// use itertools::Itertools;
//
// fn abbrev_name(name: &str) -> String {
//     name.split(" ").map(|x| x[0..1].to_uppercase()).join(".")
// }
