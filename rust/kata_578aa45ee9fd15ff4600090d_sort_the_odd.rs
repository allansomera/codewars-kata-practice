// use std::mem::replace;

fn sort_array(arr: &[i32]) -> Vec<i32> {
    // let mut v: Vec<i32> = arr
    //     .into_iter()
    //     // .filter(|x| *x % 2 == 0)
    //     .copied()
    //     .collect::<Vec<i32>>();
    // v
    let mut even_pos: Vec<usize> = Vec::new();
    let mut odd_pos: Vec<usize> = Vec::new();
    let mut odd: Vec<i32> = arr
        .into_iter()
        .filter(|x| *x % 2 != 0)
        .copied()
        .collect::<Vec<i32>>();
    // let _even: Vec<i32> = arr
    //     .into_iter()
    //     .filter(|x| *x % 2 == 0)
    //     .copied()
    //     .collect::<Vec<i32>>();
    let mut new_arr = vec![0; arr.len()];

    for (i, el) in arr.iter().enumerate() {
        if el % 2 == 0 {
            even_pos.push(i);
        } else {
            odd_pos.push(i);
        }
    }
    odd.sort();
    for (_i, el) in even_pos.iter().enumerate() {
        new_arr[*el] = arr[*el];
    }
    for (i, el) in odd_pos.iter().enumerate() {
        new_arr[*el] = odd[i];
    }
    // println!("v: {:?}", v);
    println!("even_pos: {:?}", even_pos);
    println!("odd_pos: {:?}", odd_pos);
    println!("new_var: {:?}", new_arr);
    println!("odd sorted: {:?}", odd);

    new_arr
}

fn main() {
    println!("{:?}", sort_array(&[5, 3, 2, 8, 1, 4]));
}

// soln:
// use itertools::Itertools;
//
// fn sort_array(xs: &[i32]) -> Vec<i32> {
//     let mut os = xs.iter().filter(|&x| x % 2 != 0).sorted();
//     xs.iter().map(|x| if x % 2 != 0 { os.next().unwrap() } else { x }).cloned().collect()
// }
//
// use itertools::Itertools;
//
// fn sort_array(arr: &[i32]) -> Vec<i32> {
//     let mut odd_numbers = arr
//         .iter()
//         .filter(|&x| x % 2 != 0)
//         .sorted();
//
//     return arr
//         .iter()
//         .map(|x| match x % 2 {
//                 1 => odd_numbers.next().unwrap(),
//                 _ => x,
//             })
//         .cloned()
//         .collect();
// }
//
// fn sort_array(arr: &[i32]) -> Vec<i32> {
//     let mut odd = arr
//         .iter()
//         .filter(|&x| x & 1 == 1)
//         .collect::<Vec<_>>();
//     odd.sort_by(|a, b| b.cmp(&a));
//
//     arr.iter()
//         .map(|&x| if x & 1 == 1 { *odd.pop().unwrap() } else { x })
//         .collect()
// }
//
// fn sort_array(arr: &[i32]) -> Vec<i32> {
//     let mut odds = arr.iter().filter(|&x| x % 2 == 1).collect::<Vec<&i32>>();
//     odds.sort();
//     odds.reverse();
//     arr.iter()
//         .map(|x| if x % 2 == 1 { *odds.pop().unwrap() } else { *x })
//         .collect()
// }
//
// fn sort_array(arr: &[i32]) -> Vec<i32> {
//     let mut odd = arr.iter().filter(|&x|x % 2 == 1).collect::<Vec<_>>();
//     odd.sort_by(|a, b|b.cmp(a));
//     arr.to_vec().into_iter().map(|x| if x % 2 == 0 { x } else { *odd.pop().unwrap() }).collect()
// }
//
// fn sort_array(arr: &[i32]) -> Vec<i32> {
//     let mut odd_list = arr
//         .iter()
//         .filter(|&x| x % 2 != 0)
//         .copied()
//         .collect::<Vec<_>>();
//     odd_list.sort_unstable();
//
//     let mut answer = vec![];
//     let mut odd_index = 0;
//
//     for i in arr.iter() {
//         if i % 2 != 0 {
//             answer.push(odd_list[odd_index]);
//             odd_index += 1;
//         } else {
//             answer.push(*i);
//         }
//     }
//     answer
// }
