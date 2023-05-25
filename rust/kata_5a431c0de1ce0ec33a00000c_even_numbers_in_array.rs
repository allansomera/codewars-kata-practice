fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let even: Vec<i32> = array.iter().filter(|&x| x % 2 == 0).cloned().collect();
    (&even[(even.len() - number)..]).to_vec()
}

fn main() {
    println!("{:?}", even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3));
}

// i did it with a for loop first, but decided to use filter instead
// fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
//     let mut b: Vec<i32> = Vec::new();
//     for i in array.into_iter() {
//         if i % 2 == 0 {
//             b.push(*i);
//         }
//     }
//     (&b[(b.len() - number)..]).to_vec()
// }

// soln:
// fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
//     array
//         .iter()
//         .rev()
//         .filter(|n| n.checked_rem(2) == Some(0))
//         .take(number)
//         .cloned()
//         .collect::<Vec<i32>>()
//         .into_iter()
//         .rev()
//         .collect()
// }
//
// fn even_numbers(xs: &Vec<i32>, n: usize) -> Vec<i32> {
//     let mut res: Vec<i32> = xs.iter().rev().cloned().filter(|x| x % 2 == 0).take(n).collect();
//     res.reverse();
//     res
// }
//
// fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
//     array
//         .iter()
//         .rev()
//         .filter(|&&elem| elem % 2 == 0)
//         .take(number)
//         .collect::<Vec<_>>()
//         .iter()
//         .rev()
//         .map(|&&x| x)
//         .collect()
// }
//
// fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
//     let result = array.into_iter().map(|&x|x).filter(|corr| if corr % 2 == 0 { true} else {false}).collect::<Vec<_>>();
//     result[(result.len() as usize - number)..].to_vec()
// }
