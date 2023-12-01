fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = arr
        .into_iter()
        .filter(|x| *x % 2 == 0)
        .copied()
        .collect::<Vec<i32>>();
    println!("v: {:?}", v);
    v
}

fn main() {
    println!("{:?}", sort_array(&[5, 3, 2, 8, 1, 4]));
}
