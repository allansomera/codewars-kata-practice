fn evens_and_odds(n: u64) -> String {
    if n % 2 == 0 {
        format!("{:b}", n)
    } else {
        format!("{:x}", n)
    }
}

fn main() {
    println!("{:?}", evens_and_odds(2));
    println!("{:?}", evens_and_odds(13));
    println!("{:?}", evens_and_odds(47));
    println!("{:?}", evens_and_odds(12800));
    println!("{:?}", evens_and_odds(8172381723));
}

// soln:
// fn evens_and_odds(n: u64) -> String {
//     if n % 2 == 0 { format!("{:b}", n) } else { format!("{:x}", n) }
// }
//
// fn evens_and_odds(n: u64) -> String {
//     match n%2 {
//         1 => format!("{:x}", n),
//         _ => format!("{:b}", n),
//     }
// }
//
// pub fn evens_and_odds(n: u64) -> String {
//   match n % 2 {
//     0 => format!("{n:b}"),
//     _ => format!("{n:x}"),
//   }
// }
//
// fn evens_and_odds(n: u64) -> String {
//     if n & 1 == 0 { format!("{:b}", n) } else { format!("{:x}", n) }
// }
