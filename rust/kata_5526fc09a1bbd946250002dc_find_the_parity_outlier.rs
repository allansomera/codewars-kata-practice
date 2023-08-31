use std::collections::HashMap;

fn find_outlier(values: &[i32]) -> i32 {
    // let hs: HashMap<i32, i32> = HashMap::new();
    let hs: HashMap<i32, i32> = values.iter().fold(HashMap::new(), |mut m, c| {
        if c % 2 == 0 {
            *m.entry(0).or_insert(0) += 1;
        } else {
            *m.entry(1).or_insert(0) += 1;
        }
        m
    });
    println!("{:?}", values);

    println!("hs 0: {:?}", *hs.get(&0).unwrap());
    println!("hs 1: {:?}", *hs.get(&1).unwrap());
    if *hs.get(&0).unwrap() > *hs.get(&1).unwrap() {
        println!(
            "pos even: {:?}",
            values.iter().position(|&x| x.rem_euclid(2) == 1).unwrap()
        );
        values[values
            .into_iter()
            .position(|&x| x.rem_euclid(2) == 1)
            .unwrap() as usize]
    } else {
        println!(
            "pos odd: {:?}",
            values.iter().position(|&x| x.rem_euclid(2) == 0).unwrap()
        );
        values[values.iter().position(|&x| x.rem_euclid(2) == 0).unwrap() as usize]
    }
    // println!("{:?}", hs);
    // 1
    // for i in values.iter() {}
}

fn main() {
    // println!("{:?}", find_outlier(&[2, 9, 4, 8, 6, 10, 12]));
    // println!("{:?}", find_outlier(&[2, -6, 8, -10, -3]));
    println!("{:?}", find_outlier(&[-3, 2, 6, 8, 10]));
}

// soln:
// fn find_outlier(list: &[i32]) -> i32 {
//     let parity = match list.iter().take(3).map(|x| x.abs() % 2).sum::<i32>() < 2 {
//         true => 0,
//         false => 1,
//     };
//
//     *list.iter().find(|x| x.abs() % 2 != parity).unwrap()
// }
//
// fn find_outlier(vs: &[i32]) -> i32 {
//     let rem = (vs[0] & 1) + (vs[1] & 1) + (vs[2] & 1) >> 1;
//
//     *vs.iter().find(|&v| v & 1 != rem).unwrap()
// }
//
// fn find_outlier(s: &[i32]) -> i32 {
//     let is_even = |x: &&_| *x % 2 == 0;
//     let is_odd = |x: &&_| *x % 2 != 0;
//     *s.iter()
//         .find(if s.iter().take(3).filter(is_even).count() > 1 {
//             is_odd
//         } else {
//             is_even
//         })
//         .unwrap()
// }
//
// fn find_outlier(values: &[i32]) -> i32 {
//     let even = (values.iter().filter(|x| *x % 2 == 0).count() != 1) as i32;
//     *values.iter().filter(|x| x.abs() % 2 == even).next().unwrap()
// }
//
// use itertools::Itertools;
//
// fn find_outlier(xs: &[i32]) -> i32 {
//     let mut b: Vec<&i32> = xs.iter().sorted_by(|&x, &y| (x & 1).cmp(&(y & 1))).collect();
//     if b[0] & 1 != b[1] & 1 { *b[0] } else { *b[xs.len()-1] }
// }
//
// fn find_outlier(values: &[i32]) -> i32 {
//     let (even, odd): (Vec<i32>,Vec<i32>) = values.iter().partition(|&x| x%2==0);
//     match odd.len()==1{
//         false=> even[0],
//         true=> odd[0],
//     }
// }
//
// fn find_outlier(values: &[i32]) -> i32 {
//     *values.iter().find(|&&n| n & 1 == 1^(((values[0]&1)+(values[1]&1)+(values[2]&1))>>1)).unwrap()
// }
//
// fn find_outlier(values: &[i32]) -> i32 {
//     return if (values.into_iter().filter(|&x| x.rem_euclid(2) == 1).count() == 1) {*values.into_iter().filter(|&x| x.rem_euclid(2) == 1).collect::<Vec<&i32>>()[0]} else {*values.into_iter().filter(|&x| x.rem_euclid(2) == 0).collect::<Vec<&i32>>()[0]};
// }
//
//
