use std::collections::HashSet;

fn thirt(n: i64) -> i64 {
    let repeat = vec![1, 10, 9, 12, 3, 4];
    // let num: Vec<i64> = n
    let mut sum_num = HashSet::new();
    let mut ans: i64 = n;
    while sum_num.insert(ans) {
        println!("begin of ans: {:?}", ans);
        let num: i64 = ans
            .to_string()
            .chars()
            .map(|x| x.to_string().parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, x)| {
                let repeat_idx = i % repeat.len();
                let temp = x * repeat[repeat_idx];
                temp
                // println!("{:?}", x);
                // println!("{} % {}", i, repeat.len());
                // println!("repeat_idx: {}", repeat_idx);
                // println!("repeat[{}] = {}", repeat_idx, repeat[repeat_idx]);
            })
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();

        println!("num: {:?}", num);
        // sum_num.push(num);
        ans = num;
    }

    println!("{:?}", sum_num);
    // println!("{:?}", num);
    // for (i, x) in num.iter().enumerate() {
    //     let repeat_idx = i % repeat.len();
    //
    //     println!("{} % {}", i, repeat.len());
    //     println!("repeat_idx: {}", repeat_idx);
    //     println!("repeat[{}] = {}", repeat_idx, repeat[repeat_idx]);
    // }
    // let sum: i64 = num.iter().sum();
    // println!("{}", num);

    ans
}

fn main() {
    println!("{:?}", thirt(1234567));
}

// soln:
// fn thirt(n: i64) -> i64 {
//     let pattern = [1, 10, 9, 12, 3, 4];
//
//     let m = DigitsIterator(n)
//         .zip(pattern.iter().cycle())
//         .map(|(d, p)| d * p)
//         .sum();
//
//     if m == n {
//         m
//     } else {
//         thirt(m)
//     }
// }
//
// struct DigitsIterator(i64);
//
// impl Iterator for DigitsIterator {
//     type Item = i64;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0 > 0 {
//             let d = self.0 % 10;
//             self.0 /= 10;
//             Some(d)
//         } else {
//             None
//         }
//     }
// }
//
// const REMAINDERS: [i64;6] = [1%13, 10%13, 100%13, 1_000%13, 10_000%13, 100_000%13];
//
// fn thirt(value: i64) -> i64{
//     let result: i64 = REMAINDERS.into_iter().cycle().scan(value,
//         |remaining_value, remaining_pattern| {
//             if *remaining_value<=0 {
//                 None
//             } else {
//                 let remainder = *remaining_value%10;
//                 *remaining_value /= 10;
//                 Some((remainder%10) * remaining_pattern)
//             }
//         }
//     ).sum();
//
//     if value == result {
//       result
//     } else {
//       thirt(result)
//     }
// }
//
// fn thirt(n: i64) -> i64 {
//     let remainders = [1, 10, 9, 12, 3, 4];
//     let out = remainders
//         .iter()
//         .cycle()
//         .zip(n.to_string().chars().rev())
//         .map(|(a, c)| (a * c.to_digit(10).unwrap()) as i64)
//         .sum();
//     if out == n {
//         out
//     } else {
//         thirt(out)
//     }
// }
//
// fn thirt(n: i64) -> i64{
//     let mut n = n;
//
//     let v = vec![1, 10, 9, 12, 3, 4];
//
//     loop {
//         let new_n = n.to_string().chars()
//             .map(|d| d.to_digit(10).unwrap() as i64)
//             .rev()
//             .zip(v.iter().cycle())
//             .fold(0, |acc, el| acc + el.0 * el.1);
//
//         if new_n == n {
//             return new_n;
//         }
//         n = new_n
//     }
// }
//
// fn next_num(n: i64) -> i64 {
//     (0..=18u32).map(|i| n / 10i64.pow(i) % 10 * (10i64.pow(i) % 13)).sum()
// }
//
// fn thirt(n: i64) -> i64{
//     std::iter::successors(Some(n), |n| Some(next_num(*n)).filter(|_| *n >= 100)).last().unwrap_or(n)
// }
//
// fn thirt(n: i64) -> i64 {
//     let mut result = sum(n);
//     loop {
//         let new_result = sum(result);
//         if result == new_result {
//             return result;
//         }
//         result = new_result;
//     }
// }
//
// fn sum(n: i64) -> i64 {
//     std::iter::repeat(0)
//         .scan(n, |n, _| match *n {
//             0 => None,
//             _ => {
//                 let digit = *n % 10;
//                 *n /= 10;
//                 Some(digit)
//             }
//         })
//         .zip([1, 10, 9, 12, 3, 4].iter().cycle())
//         .map(|(x, y)| x * y)
//         .sum()
// }
//
// fn thirt(n: i64) -> i64{
// 	let th = [1,10,9,12,3,4_i64]
//   	.iter()
//     .cycle()
//   	.zip(digits(n).iter())
//     .map(|(a, b)| a * b)
//     .fold(0, |a, b| a + b);
//   if th == n { th } else { thirt(th) }
// }
//
// fn digits(n: i64) -> Vec<i64> {
// 	let mut ret: Vec<i64> = Vec::new();
//   let mut m = n;
//   while m > 0 {
//   	ret.push(m % 10);
//     m /= 10;
//   }
//   ret
// }
//
// fn thirt(n: i64) -> i64{
//
//   let  ww = n
//         .to_string()
//         .chars()
//         .rev()
//         .enumerate()
//         .map(|(i,x)| {
//
//           let a = x as i64 - 0x30;
//           let b = 10_i64.pow(i as _ ) % 13;
//           let res = a * b;
//
//
//           res
//         }).sum();
//
//
//
//
//   if ww  == n {ww } else {thirt(ww)}
// }
