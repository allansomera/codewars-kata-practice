use std::collections::HashMap;
fn solve(vec: &[i32]) -> Vec<i32> {
    let mut c: Vec<i32> = vec.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();
    let mut cnt = HashMap::new();
    for (i, el) in c.iter().enumerate() {
        cnt.insert(*el, cnt.get(&el).unwrap_or(&0) + 1);
    }
    let mut cs = cnt.clone().into_iter().collect::<Vec<(i32, i32)>>();
    cs.sort_unstable_by(|(_, a), (_, b)| b.cmp(&a));
    cs.clone()
        .into_iter()
        .map(|(c, v)| vec![c; v as usize].into_iter().collect::<Vec<i32>>())
        .collect::<Vec<_>>();

    for (_, el) in cs.iter().enumerate() {
        // println!("el: {:?}", *el);
        // for (_, e) in *el {
        //     println!("e: {:?}", *e);
        // }
        println!("0: {:?}", el.0);
        println!("1: {:?}", el.1);
    }

    println!("c: {:?}", c);
    println!("cnt: {:?}", cnt);
    println!("cs: {:?}", cs);
    c
}

fn main() {
    println!("{:?}", solve(&vec![2, 3, 5, 3, 7, 9, 5, 3, 7]));
}
