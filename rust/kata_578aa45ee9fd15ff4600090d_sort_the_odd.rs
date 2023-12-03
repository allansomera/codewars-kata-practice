fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = arr
        .into_iter()
        // .filter(|x| *x % 2 == 0)
        .copied()
        .collect::<Vec<i32>>();
    // v
    let mut pos: Vec<usize> = Vec::new();
    let odd: Vec<i32> = arr
        .into_iter()
        .filter(|x| *x % 2 != 0)
        .copied()
        .collect::<Vec<i32>>();
    let new_var = vec![0; arr.len()];

    for (i, el) in arr.iter().enumerate() {
        if el % 2 == 0 {
            pos.push(i);
        }
    }
    println!("v: {:?}", v);
    println!("pos: {:?}", pos);
    println!("odd: {:?}", odd);

    v
}

fn main() {
    println!("{:?}", sort_array(&[5, 3, 2, 8, 1, 4]));
}
