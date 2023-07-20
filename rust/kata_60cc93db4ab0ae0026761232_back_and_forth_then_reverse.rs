fn arrange(s: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    if s.len() == 2 {
        return s.to_vec();
    } else if s.len() == 0 {
        return vec![];
    }

    let mut a = 0 as usize;
    let mut b = s.len() - 1 as usize;

    let cycles: i32;
    if s.len() % 2 == 0 {
        cycles = (s.len() as i32) / 2 as i32;
    } else {
        cycles = ((s.len() as i32) - 1) / 2 as i32;
    }
    println!("cycles: {:?}", cycles);

    // while tmp.len() >= 2 {
    //     // println!("{:?}", tmp);
    //     v.push(tmp.remove(0));
    //     v.push(tmp.remove(tmp.len() - 1));
    //     tmp.reverse();
    //     // tmp.sort_by(|a, b| Reverse(a).cmp(&Reverse(b)));
    //     println!("tmp: {:?}", tmp);
    // }
    for x in 0..cycles {
        println!("a: {} b: {}", a, b);
        println!("before s: {:?}", &s[a..=b]);
        if x % 2 == 0 {
            v.push(s[a]);
            v.push(s[b]);
            // (a, b) = (b, a);
        } else {
            v.push(s[b]);
            v.push(s[a]);
            // println!("s: {:?}", &s[a..=b]);
            // println!("v: {:?}", v);
            // (a, b) = (b, a);
            // println!("a: {} b: {}", a, b);
        }
        a = a + 1 as usize;
        b = b - 1 as usize;
        println!("after s: {:?}", &s[a..=b]);
        // println!("a: {} b: {}", a, b);
        println!("v: {:?}", v);
    }
    if (s[a..=b].len()) as usize == 1 {
        v.push(s[a]);
    }
    v
}

fn main() {
    // println!("{:?}", arrange(&vec![1, 2]));
    // println!("{:?}", arrange(&vec![4, 3, 2]));
    // println!("{:?}", arrange(&vec![9, 7, -2, 8, 5, -3, 6, 5, 1]));
    println!("{:?}", arrange(&vec![2, 4, 3, 4]));
}

// soln:
// fn arrange(xs: &[i32]) -> Vec<i32> {
//     type It<'a> = std::slice::Iter::<'a, i32>;
//     [It::next, It::next_back, It::next_back, It::next]
//         .into_iter().cycle().scan(xs.iter(), |ix, f| f(ix).cloned()).collect()
// }
//
// fn arrange(s: &[i32]) -> Vec<i32> {
//     (0..s.len())
//         .map(|i| match i % 4 {
//             0 | 3 => s[i / 2],
//             _ => s[s.len() - i / 2 - 1],
//         })
//         .collect()
// }
//
// fn arrange(s: &[i32]) -> Vec<i32> {
//     let s_len: usize = (s.len() + (2-1)) / 2;
//     let mut result: Vec<i32> = Vec::new();
//     let mut insert_order : bool = true;
//     (0..s_len).enumerate().for_each(|(i, _)| {
//         if insert_order {
//             result.push(s[i]);
//             result.push(s[s.len() - 1 - i]);
//         }
//         else {
//             result.push(s[s.len() - 1 - i]);
//             result.push(s[i]);
//         }
//         insert_order = !insert_order;
//         if (i+1) == s_len && s.len() % 2 != 0 {result.pop();}
//     });
//     return result;
// }
//
// pub fn arrange(s: &[i32]) -> Vec<i32> {
//     if s.is_empty() {
//         return vec![];
//     }
//     let len = s.len();
//     let (mut front, mut back) = (0, len);
//     let mut get_source = |idx| -> usize {
//         match idx % 4 {
//             0 | 3 => {
//                 front += 1;
//                 front
//             }
//             _ => {
//                 back -= 1;
//                 back
//             }
//         }
//     };
//     let mut result = vec![s[0]; len];
//     for idx in 1..len {
//         result[idx] = s[get_source(idx)];
//     }
//     result
// }
//
// fn arrange(xs: &[i32]) -> Vec<i32> {
//     let mut it = xs.iter();
//     (0..xs.len())
//         .flat_map(|i| match i % 4 {
//             1 | 2 => it.next_back().cloned(),
//             _ => it.next().cloned(),
//         })
//         .collect()
// }
//
//
