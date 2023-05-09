fn points(games: &[String]) -> u32 {
    let gg = games.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let sum: u32 = gg
        .iter()
        .map(|x| {
            let scores: Vec<&str> = x.split(":").collect();
            let a = scores[0].parse::<u32>().unwrap();
            let b = scores[1].parse::<u32>().unwrap();
            if a > b {
                3
            } else if b > a {
                0
            } else {
                1
            }
        })
        .sum();
    sum
}

fn main() {
    println!(
        "{:?}",
        points(
            &["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .as_slice()
        )
    );
}

// soln:
// fn points(games: &[String]) -> u32 {
//     let mut scorex: u32 = 0;
//
//     for game in games {
//         let x: &u32 = &game[..1].parse().unwrap();
//         let y: &u32 = &game[2..].parse().unwrap();
//
//         if x > y {
//             scorex += 3;
//         } else if x == y {
//             scorex += 1;
//         }
//     }
//     scorex
// }
// // ==
// fn get_points(s: &String) -> u32 {
//     let mut it = s.split(":");
//     let x = it.next().unwrap().parse::<u8>().unwrap();
//     let y = it.next().unwrap().parse::<u8>().unwrap();
//
//     match x {
//         _ if x > y => 3,
//         _ if x == y => 1,
//         _ => 0,
//     }
// }
//
// fn points(games: &[String]) -> u32 {
//     games.iter().map(get_points).sum()
// }
// // ==
//
// fn points(games: &[String]) -> u32 {
//     let mut total: u32 = 0;
//     for x in games.iter(){
//         let first = &x[0..1];
//         let last = &x[2..];
//         if first > last      {total += 3}
//         else if first < last {total += 0}
//         else                 {total += 1}
//     }
//     total
// }
// fn points(games: &[String]) -> u32 {
//     games.iter().fold(0, |acc, game| {
//         let game = game.split(":")
//         .map(|x| x.parse().unwrap())
//         .collect::<Vec<u32>>();
//         acc + match game.as_slice() {
//             [x, y] if x > y => 3,
//             [x, y] if x < y => 0,
//             [x, y] if x == y => 1,
//             _ => 0
//         }
//     })
// }
//
// fn points(games: &[String]) -> u32 {
//     let mut point = 0;
//     for i in games.iter() {
//         if i[..1] == i[2..] {
//             point += 1;
//         }
//         if i[..1] > i[2..] {
//             point += 3;
//         }
//     }
//     point
// }
