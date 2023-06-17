fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
    numbers.iter().map(|x| *x.iter().min().unwrap()).sum()
}

fn main() {
    println!(
        "{:?}",
        sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 3]])
    );
}

// soln:
// fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
//     numbers.iter().map(|row| row.iter().min().unwrap()).sum()
// }
//
// fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
//     numbers.iter().filter_map(|v| v.iter().min()).sum()
// }
//
// fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
//    numbers.iter().fold(0, |a, x| a + x.iter().min().unwrap())
// }
