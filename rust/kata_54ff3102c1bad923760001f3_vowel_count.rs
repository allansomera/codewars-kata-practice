fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let v = ['a', 'e', 'i', 'o', 'u'];

    for s in string.chars() {
        if v.iter().any(|&v| s.to_string().starts_with(v)) {
            vowels_count += 1;
        }
    }
    vowels_count
}

fn main() {
    println!("{:?}", get_count("aaaaaa"));
}

// soln:
// fn get_count(string: &str) -> usize {
//     string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
// }
//
// fn get_count(string: &str) -> usize {
//     string
//         .chars()
//         .filter(|&c| "aeiou".contains(c))
//         .count()
// }
//
// fn get_count(s: &str) -> usize {
//   s.matches(&['a', 'e', 'i', 'o', 'u'][..]).count()
// }
//
// fn get_count(string: &str) -> usize {
//     string.matches(|x| "aeiou".contains(x)).count()
// }
//
// fn get_count(string: &str) -> usize {
//     string
//         .chars()
//         .filter(|&x| "aeiou".chars().any(|y| x == y))
//         .count()
// }
//
// fn get_count(string: &str) -> usize {
//     let vowels = ['a', 'e', 'i', 'o', 'u'];
//     let mut vowels_count: usize = 0;
//     for char in string.chars() {
//         if vowels.contains(&char) {
//             vowels_count += 1;
//         }
//     }
//     vowels_count
// }
//
// fn get_count(string: &str) -> usize {
//     let vowels = ['a', 'e', 'i', 'o', 'u'];
//     string.chars().filter(|letter| vowels.contains(letter)).collect::<Vec<char>>().iter().count()
//
// }
