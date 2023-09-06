fn solve(arr: &[u32; 3]) -> u32 {
    // let mut day_count: u32 = 0;
    // let mut chips: u32 = 0;
    let mut b = arr.clone();
    // let mut keep_going = true;
    b.sort();
    //
    // let max_val = b.into_iter().max().unwrap();
    // println!("max val: {:?}", max_val);
    //
    // while keep_going {
    //     // for i in b.iter() {}
    //     for (i, el) in b.into_iter().enumerate() {
    //         if chips <= 2 && *el != 0 as u32 {
    //             b[i] -= 1;
    //         } else if chips == 2u32 {
    //             day_count += 1u32;
    //             chips = 0;
    //             keep_going = false;
    //         } else {
    //             continue;
    //         }
    //     }
    // }
    // println!("{:?}", b);
    // 1
    if b[2] <= (b[0] + b[1]) {
        b.iter().sum::<u32>() / 2
    } else {
        b[0] + b[1]
    }
}

fn main() {
    println!("{:?}", solve(&[4, 1, 1]));
    println!("{:?}", solve(&[1, 1, 1]));
    println!("{:?}", solve(&[3, 3, 3]));
}

// soln:
// use std::cmp;
//
// fn solve(arr: &[u32; 3]) -> u32 {
//     let mut xs = arr.to_vec();
//     xs.sort();
//     let a = xs[0];
//     let b = xs[1];
//     let c = xs[2];
//     return cmp::min(a + b, (a + b + c) / 2);
// }
//
// fn solve(ns: &[u32; 3]) -> u32 {
//     let [nw, ng, nb] = ns;
//     ((nw + ng + nb) - nw.max(ng).max(nb)).min((nw + ng + nb) / 2)
// }
//
// fn solve(arr: &[u32; 3]) -> u32 {
//     let sum: u32 = arr[0] + arr[1] + arr[2];
//     let max: u32 = arr[0].max(arr[1]).max(arr[2]);
//     if sum < max * 2 { sum - max } else { sum / 2 }
// }
//
// fn solve(arr: &[u32; 3]) -> u32 {
//     let mut d = 0;
//     let mut v = arr.to_vec();
//     v.sort();
//
//     while v[0] != 0 || v[1] != 0 {
//         let x = if v[1] > 1 { v[1] / 2 } else { 1 };
//         v[1] -= x;
//         v[2] -= x;
//         d += x;
//         v.sort();
//     }
//     d
// }
//
// fn solve(arr: &[u32; 3]) -> u32 {
//     let m = arr.iter().max().unwrap();
//     let s = arr.iter().sum();
//     if 2*m>=s {
//         s-m
//     } else {
//         s/2
//     }
// }

// explanation:
// https://codeforces.com/blog/entry/71844?locale=en
