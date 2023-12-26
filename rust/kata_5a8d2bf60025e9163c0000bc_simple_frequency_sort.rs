// use itertools::Itertools;
// use std::cmp::Ordering;
use std::collections::HashMap;

fn solve(vec: &[i32]) -> Vec<i32> {
    let mut c: Vec<i32> = vec.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();
    let mut cnt = HashMap::new();
    for (i, el) in c.iter().enumerate() {
        cnt.insert(*el, cnt.get(&el).unwrap_or(&0) + 1);
    }
    let mut cs = cnt.clone().into_iter().collect::<Vec<(i32, i32)>>();
    cs.sort_by(|(_, a), (_, b)| b.cmp(&a));
    // let sorted_cs = cs.into_iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1));
    //sort by duplicates first and then descending order
    c.sort_by(|a, b| {
        let count_a = cnt[a];
        let count_b = cnt[b];
        // compare by (duplicates first)
        count_b.cmp(&count_a).then_with(|| a.cmp(b))
    });
    // println!("cs: {:?}", cs);
    // cs.clone()
    //     .into_iter()
    //     .map(|(c, v)| vec![c; v as usize].into_iter().collect::<Vec<i32>>())
    //     .collect::<Vec<_>>();
    //
    // let mut freq: Vec<i32> = Vec::new();
    //
    // for (_, el) in cs.iter().enumerate() {
    //     // println!("el: {:?}", *el);
    //     // for (_, e) in *el.iter().enumerate() {
    //     //     println!("e: {:?}", *e);
    //     // }
    //     for _ in 0..el.1 {
    //         freq.push(el.0);
    //     }
    //     println!("0: {:?}", el.0);
    //     println!("1: {:?}", el.1);
    // }
    //
    // println!("c: {:?}", c);
    // println!("cnt: {:?}", cnt);
    // println!("cs: {:?}", cs);
    // println!("freq: {:?}", freq);
    c
}

fn main() {
    println!("{:?}", solve(&vec![2, 3, 5, 3, 7, 9, 5, 3, 7]));
}

soln:
use std::collections::HashMap;

fn solve(vec: &[i32]) -> Vec<i32> {
    let counts = vec.iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });
    
    let mut nums = vec.to_vec();
    nums.sort_by(|a, b| {
        let x = counts.get(a).unwrap();
        let y = counts.get(b).unwrap();
        y.cmp(x).then(a.cmp(b))
    });
    
    nums
}

use itertools::{Itertools, repeat_n};
use std::cmp::Reverse;

fn solve(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().counts().into_iter()
        .sorted_by_key(|&(x, f)| (Reverse(f), x))
        .flat_map(|(x, f)| repeat_n(x, f))
        .collect()
}

use std::{collections::HashMap, cmp::Ordering};

fn solve(vec: &[i32]) -> Vec<i32> {
    let mut counter:HashMap<i32, i32> = HashMap::new();
    let mut vec = vec.to_vec();
    vec.iter().for_each(|&n| *counter.entry(n).or_insert(0) += 1);
    vec.sort_by(|a, b| match counter[b].cmp(&counter[a]) {
        Ordering::Equal => a.cmp(b),
        r => r,
    });
    vec
}

fn solve(vec: &[i32]) -> Vec<i32> {
    use std::collections::HashMap;
    let mut histogram = HashMap::with_capacity(vec.len());
    for n in vec.iter() {
        *histogram.entry(n).or_insert(0) += 1;
    }
    
    let mut res: Vec<i32> = vec.iter().cloned().collect();
    res.sort_unstable_by(|a, b| {
            let af = histogram.get(a);
            let bf = histogram.get(b);
            if af == bf { a.cmp(&b) } else { bf.cmp(&af) }
        }
    );
    
    res
}

use itertools::Itertools;

fn solve(ns: &[i32]) -> Vec<i32> {
    ns.iter()
        .sorted()
        .group_by(|n| *n)
        .into_iter()
        .map(|(_, ns)| ns.collect::<Vec<_>>())
        .sorted_by_key(|ns| -(ns.len() as isize))
        .flatten()
        .map(|n| *n)
        .collect()
}

use itertools::Itertools;

fn solve(vec: &[i32]) -> Vec<i32> {
    let freq = vec.iter().counts();
    
    vec.iter()
       .sorted_unstable_by(|a, b| freq[b].cmp(&freq[a]).then(a.cmp(b)))
       .cloned()
       .collect()
}

fn solve(orig: &[i32]) -> Vec<i32> {
    let mut vec = orig.to_owned();
    vec.sort_unstable();
    vec.reverse();
    vec.sort_by_key(|x| orig.iter().filter(|&y| x == y).count());
    vec.reverse();
    vec
}
