use std::collections::HashMap;
fn solve(vec: &[i32]) -> Vec<i32> {
    let mut c: Vec<i32> = vec.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();
    let mut cnt = HashMap::new();
    for (i, el) in c.iter().enumerate() {
        cnt.insert(*el, cnt.get(&el).unwrap_or(&0) + 1);
    }
    let mut cs = cnt.into_iter().collect::<Vec<(i32, i32)>>();
    cs.sort_unstable_by(|(_, a), (_, b)| b.cmp(&a));
    cs.into_iter()
        .map(|(c, v)| vec![c; v as usize].into_iter().collect::<i32>())
        .collect();

    println!("c: {:?}", c);
    println!("cs: {:?}", cs);
    c
}

fn main() {
    println!("{:?}", solve(&vec![3, 3, 3, 5, 5, 7, 7, 2, 9]));
}
