fn sum_cubes(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(3)).sum()
}

fn main() {
    println!("{:?}", sum_cubes(2));
}

// soln:
// fn sum_cubes(n: u32) -> u32 {
//     if (n == 0){
//         return 0;
//     }
//     return n * n * n + sum_cubes(n - 1);
// }
//
// fn sum_cubes(n: u32) -> u32 {
//     return (n.pow(2) * (n + 1).pow(2)) / 4;
// }
//
// fn sum_cubes(n: u32) -> u32 {
//     (1..=n).map(|e| e*e*e).sum()
// }
