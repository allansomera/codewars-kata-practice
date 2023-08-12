fn play_pass(s: &str, n: u32) -> String {
    // let ss: Vec<String> = s.chars().map(|x| x.to_string()).collect::<Vec<String>>();
    // .into_iter()
    // .rev()
    // .collect::<Vec<_>>()
    // .into_iter()
    // .map(|x| x.chars().rev().collect::<String>())
    // .map(|x| x.to_string())
    // .collect();

    // let letter = 'b';
    // print!("{}:{} => ", letter, 'a' as u8 + (letter as u8 - 'a' as u8));
    // println!(
    //     "{}:{}",
    //     ('a' as u8 + ((letter as u8 - 'a' as u8) + 1) % 26) as char,
    //     ('a' as u8 + ((letter as u8 - 'a' as u8) + 1) % 26)
    // );
    //
    // println!("{:?}", ss);
    let mut c: char = 'a';
    s.chars()
        .map(|ch| {
            match ch {
                'A'..='Z' => c = ('A' as u8 + ((ch as u8 - 'A' as u8) + n as u8) % 26) as char,
                'a'..='z' => c = ('a' as u8 + ((ch as u8 - 'a' as u8) + n as u8) % 26) as char,
                ' ' => c = ch,
                ch if ch.is_ascii_digit() => {
                    c = ((9 - (ch.to_string().parse::<i32>().unwrap() as i32))
                        .to_string()
                        .chars()
                        .next()
                        .unwrap()) as char
                }
                _ => c = ch,
            }
            c
        })
        .collect::<String>()
        // .chars()
        // .rev()
        // .collect::<String>()
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if x.is_ascii_alphabetic() {
                if i % 2 == 1 {
                    return x.to_string().to_lowercase();
                } else {
                    return x.to_string().to_uppercase();
                }
            } else {
                return x.to_string();
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
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
    println!("{:?}", play_pass("BORN IN 2015!", 1));
    println!("{:?}", play_pass("~J 2F!1S#!)H   C Y OOQ (Z(7 ", 10));
}

soln:
fn play_pass(s: &str, n: u32) -> String {
    s.char_indices()
     .map(|(i, c)| match c {
        c if c.is_digit(10) => (105 - c as u8) as char,
        c if c.is_alphabetic() => {
             let tmp = (65 + (c as u32 - 65 + n).rem_euclid(26) as u8) as char;
             if i % 2 == 0 { tmp } else { tmp.to_ascii_lowercase() } 
         }
         _ => c,
    }).rev().collect()
}

fn play_pass(s: &str, n: u32) -> String {
    let vec: Vec<char> = s.chars().enumerate().map(|(i, c)| {
        if c.is_ascii_alphabetic() {
            ((c as u32 % 32 - 1 + n) % 26 + 65 + i as u32 % 2 * 32) as u8 as char
        } else if c.is_ascii_digit() {
            (48 + 57 - c as u32) as u8 as char
        } else { c }
    }).collect();
    vec.iter().rev().collect()
}

fn play_pass(s: &str, n: u32) -> String {
    s.char_indices()
        .map(|(i, c)| match c {
            'a'..='z' | 'A'..='Z' => match i % 2 == 0 {
                true => char::from((((c.to_ascii_uppercase() as u8 - 65) as u32 + n) % 26 + 65) as u8),
                false => char::from((((c.to_ascii_lowercase() as u8 - 97) as u32 + n) % 26 + 97) as u8),                
            },
            '0'..='9' => c.to_digit(10).and_then(|d| std::char::from_digit(9 - d, 10)).unwrap(),
            c => c,
        })
        .rev()
        .collect()
}

fn play_pass(s: &str, n: u32) -> String {
    s.chars().enumerate().map(|c| {
        // circular shift chars || replace digit
        match (c.0%2 , c.1) {
            (_,'0'..='9') => ('0' as u8 + '9' as u8 - c.1 as u8) as char , // digit
            (0,'a'..='z') => ((c.1  as u8 - 'a' as u8 + n as u8) % 26 + 'A' as u8) as char  , // even
            (0,'A'..='Z') => ((c.1 as u8 - 'A' as u8 + n as u8) % 26 + 'A' as u8) as char  , // even
            (1,'a'..='z') => ((c.1 as u8 - 'a' as u8 + n as u8) % 26 + 'a' as u8) as char  , // odd
            (1,'A'..='Z') => ((c.1 as u8 - 'A' as u8 + n as u8) % 26 + 'a' as u8) as char  , // odd
            _ => c.1,
        }
    }).fold(String::new(), |acc, c|  c.to_string() + &acc)
}
