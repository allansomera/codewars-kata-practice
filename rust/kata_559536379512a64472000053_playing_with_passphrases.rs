fn play_pass(s: &str, n: u32) -> String {
    let ss: Vec<String> = s
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect();

    // let letter = 'b';
    // print!("{}:{} => ", letter, 'a' as u8 + (letter as u8 - 'a' as u8));
    // println!(
    //     "{}:{}",
    //     ('a' as u8 + ((letter as u8 - 'a' as u8) + 1) % 26) as char,
    //     ('a' as u8 + ((letter as u8 - 'a' as u8) + 1) % 26)
    // );
    //
    // println!("{:?}", ss);
    ss.iter()
        .map(|x| {
            let mut c: char = '\0';
            x.chars()
                .map(|ch| {
                    match ch {
                        'A'..='Z' => {
                            c = ('A' as u8 + ((ch as u8 - 'A' as u8) + n as u8) % 26) as char
                        }
                        'a'..='z' => {
                            c = ('a' as u8 + ((ch as u8 - 'a' as u8) + n as u8) % 26) as char
                        }
                        // 'A'..='Z' => 'X',
                        // 'a'..='z' => 'x',
                        _ => c = ch,
                    };
                    c
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
    //
    // println!("{:?}", ss);
    // for i in ss.iter() {
    //     for x in i.chars() {
    //         // println!("{}", x)
    //         let c: char;
    //         match x {
    //             'A'..='Z' => c = 'x',
    //             _ => c = 'b',
    //         }
    //         println!("{}", c)
    //     }
    // }
    // "test".to_string()
}

fn main() {
    println!("{:?}", play_pass("BORN IN 2015", 1));
}
