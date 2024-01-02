fn interpreter(tape: &str, data: &str) -> String {
    let mut res: String = String::new();
    // res = tape.chars().map(|x| match x {
    //     "0" =>
    //
    // }).collect::<String>();
    let mut tape_idx = 0;
    let mut idx;
    let mut cursor = 0;
    let mut end: usize = tape.len();
    let mut p_end: usize = data.len();
    let mut cur = 0;
    while cur < end {}
    println!("tape: {:?}", tape);
    println!("data: {:?}", data);
    "test".to_string()
}

fn main() {
    println!("{:?}", interpreter("10", "1010101"));
}
