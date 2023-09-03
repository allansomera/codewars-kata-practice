fn solve(arr: &[u32; 3]) -> u32 {
    let mut day_count: u32 = 0;
    let mut chips: u32 = 0;
    let mut b = arr.clone();
    let mut keep_going = true;

    while keep_going {
        for (i, el) in b.into_iter().enumerate() {
            if chips < 3 && *el != 0 as u32 {
                b[i] -= 1;
            } else if chips == 2u32 {
                day_count += 1u32;
                chips = 0;
                keep_going = false;
            } else {
                continue;
            }
        }
    }
    println!("{:?}", b);
    1
}

fn main() {
    println!("{:?}", solve(&[1, 1, 1]));
}
