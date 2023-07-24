// use std::regex::Regex;

fn decode(s: &str) -> String {
    // let re = Regex::new(r"^[0-9]+").unwrap();
    // let numbers: Vec<&str> = re.find_iter(s).map(|x| x.as_str()).collect();
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let number: i32 = s
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_string())
        // .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .join("")
        .parse::<i32>()
        .unwrap();
    // .into_iter().map(|x| x.parse::<i32>.unwrap());
    let letters: String = s.chars().filter(char::is_ascii_alphabetic).collect();
    let mut num_d: Vec<i32> = Vec::new();

    for x in letters.chars() {
        for i in 0..26 {
            if (i * number) % 26 == (x as u8 - b'a') as i32 {
                // println!("found x: {}", i)
                num_d.push(i);
            }
        }
    }
    println!("num_d len: {}", num_d.len());

    if num_d.len() > letters.len() {
        return String::from("Impossible to decode");
    }
    let num_to_s: String = num_d
        .iter()
        // .map(|x| std::char::from_u32((*x as u8) as u32).unwrap().to_string())
        .map(|x| alpha.chars().nth(*x as usize).unwrap().to_string())
        .collect::<String>();
    // println!("{:?}", number);
    // println!("{:?}", letters);
    println!("{:?}", num_d);
    println!("{:?}", num_to_s);
    num_to_s
}

fn main() {
    // println!("{:?}", decode("10559625hbkeohysnztuuqdznnkkcgjndbujej"));
    // println!("{:?}", decode("123abcde"));
    println!("{:?}", decode("6015ekx"));
    println!("{:?}", decode("5057aan"));
}

// soln:
// fn decode(s: &str) -> String {
//     let digits = &s[..s.chars().take_while(|c| c.is_digit(10)).count()];
//     let num: usize = digits.parse().unwrap();
//     let mut maps = [255; 26];
//
//     for i in 0..26 {
//         let j = (num * i) % 26;
//         if maps[j] != 255 {
//             return "Impossible to decode".to_string();
//         }
//         maps[j] = i as u8;
//     }
//
//     s[digits.len()..]
//         .bytes()
//         .map(|c| 'a' as u8 + maps[(c - 'a' as u8) as usize])
//         .map(|n| n as char)
//         .collect()
// }
//
// fn decode(s: &str) -> String {
//     let num = s
//         .chars()
//         .take_while(|c| c.is_ascii_digit())
//         .fold(0, |n, c| 10 * n + c.to_digit(10).unwrap());
//
//     s.chars()
//         .skip_while(|c| c.is_ascii_digit())
//         .map(|c| (c as u8) - b'a')
//         .map(|n| {
//             (0..=25)
//                 .filter(|x| (x * num) % 26 == n.into())
//                 .map(|x| char::from(b'a' + (x as u8)))
//                 .fold((None, true), |(_, first), c| {
//                     if first {
//                         (Some(c), false)
//                     } else {
//                         (None, false)
//                     }
//                 })
//                 .0
//         })
//         .collect::<Option<String>>()
//         .unwrap_or_else(|| String::from("Impossible to decode"))
// }
//
// fn decode(s: &str) -> String {
//     let word = s
//         .chars()
//         .filter(char::is_ascii_lowercase)
//         .map(|c| c as u8 - 'a' as u8)
//         .collect::<Vec<u8>>();
//
//     let code = s
//         .chars()
//         .filter(char::is_ascii_digit)
//         .collect::<String>()
//         .parse::<u32>()
//         .unwrap();
//
//     let decoded = word.iter().fold(vec![], |mut options, &encoded| {
//         options.push(
//             (0..26)
//                 .filter(|&n| n as u32 * code % 26 == encoded as u32)
//                 .collect::<Vec<u8>>(),
//         );
//         options
//     });
//
//     if decoded.iter().any(|options| options.len() != 1) {
//         return String::from("Impossible to decode");
//     }
//
//     decoded
//         .iter()
//         .flatten()
//         .map(|&c| char::from(c + 'a' as u8))
//         .collect::<String>()
// }
//
// fn decode(s: &str) -> String {
//     let n = s.chars().take_while(|c| c.is_digit(10)).collect::<String>();
//     let k = n.len();
//     let n = n.parse::<u32>().unwrap();
//     for i in 1..26 {
//         if i * n % 26 == 1 {
//             return s[k..].chars().map(|c| ((c as u32 - 97) * i % 26 + 97) as u8 as char).collect();
//         }
//     }
//     "Impossible to decode".to_string()
// }
//
// fn decode(s: &str) -> String {
//     let digits_len = s.chars().take_while(| c | c.is_digit(10)).count();
//     let num: usize = s[..digits_len].parse().unwrap_or(0);
//     let mut maps = [255_u8; 26];
//
//     for i in 0..26 {
//         let j = (num * i) % 26;
//
//         if maps[j as usize] != 255 {
//             return "Impossible to decode".to_string();
//         }
//
//         maps[j as usize] = i as u8;
//     }
//
//     s[digits_len..].bytes()
//         .map(| c | (b'a' + maps[(c - b'a') as usize]) as char)
//         .collect()
// }
//
// fn decode(s: &str) -> String {
//     let num: u32 = s.chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap();
//     if num % 2 == 0 || num % 13 == 0 { return "Impossible to decode".into() }
//     s.bytes().skip_while(u8::is_ascii_digit)
//         .map(|b| ((0..26).find(|n| n * num % 26 == (b - 97) as u32).unwrap() + 97) as u8 as char)
//         .collect()
// }
