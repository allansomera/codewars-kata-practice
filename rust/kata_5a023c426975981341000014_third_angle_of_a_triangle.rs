fn other_angle(a: u32, b: u32) -> u32 {
    let c: u32 = 180 - (a + b);
    c
}

fn main() {
    println!("third angle: {}", other_angle(30, 60));
}

// soln:
// fn other_angle(a: u32, b: u32) -> u32 {
//     180 - a - b
// }
//
// fn other_angle(a: u32, b: u32) -> u32 {
//      ((a+b) as i32 - 180).abs() as u32
// }
