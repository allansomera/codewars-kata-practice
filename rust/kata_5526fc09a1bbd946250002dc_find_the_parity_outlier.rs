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
    if *hs.get(&0).unwrap() > *hs.get(&1).unwrap() {
        println!(
            "pos even: {:?}",
            values.into_iter().position(|&x| x % 2 == 1)
        );
        values[values.iter().position(|&x| x % 2 == 1).unwrap() as usize]
    } else {
        println!(
            "pos odd: {:?}",
            values.into_iter().position(|&x| x % 2 == 1)
        );
        values[values.iter().position(|&x| x % 2 == 0).unwrap() as usize]
    }
    // println!("{:?}", hs);
    // 1
    // for i in values.iter() {}
}

fn main() {
    // println!("{:?}", find_outlier(&[2, 9, 4, 8, 6, 10, 12]));
    println!("{:?}", find_outlier(&[2, -6, 8, -10, -3]));
}
