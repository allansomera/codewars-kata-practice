fn solve(vec: &[i32]) -> Vec<i32> {
    let mut c: Vec<i32> = vec.into_iter().map(|x| *x as i32).collect::<Vec<i32>>();
    c.sort();
    println!("c: {:?}", c);
    c
}

fn main() {
    println!("{:?}", solve(&vec![3, 3, 3, 5, 5, 7, 7, 2, 9]));
}
