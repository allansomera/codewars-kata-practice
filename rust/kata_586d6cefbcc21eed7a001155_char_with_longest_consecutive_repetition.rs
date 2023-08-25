extern crate fancy_regex;
use fancy_regex::Regex;

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.is_empty() {
        return None;
    }
    let re = Regex::new(r"(.)(\1*)").unwrap();
    // let Some(cap) = re.captures(s) else {None};
    // if let Ok(Some(cap)) = re.captures(s) {
    //     println!("cap: {:?}", cap);
    //     return Some((cap[0].chars().next().unwrap(), cap.len()));
    // }
    // let result = re.captures(s).unwrap().unwrap();
    // println!("{:?}", result.unwrap());
    // assert!(result.is_ok());
    // let did_match = result.unwrap();
    // println!("{:?}", result.get(1).unwrap().as_str());

    let mut max_char: Option<char> = None;
    let mut max_count: Option<usize> = None;

    // println!("{:?}", re.captures_len());
    for capture in re.captures_iter(s) {
        // println!("capture: {:?}", );
        // println!("capture[1]: {:?}", capture[1]);
        if let Ok(capture) = capture {
            let curr_char: Option<char> =
                Some(capture.get(0).unwrap().as_str().chars().next().unwrap());
            let curr_text = capture.get(0).unwrap().as_str();
            let curr_count: Option<usize> = Some(curr_text.len());
            if max_count == None {
                max_count = curr_count;
            }
            if max_char == None {
                max_char = curr_char;
            }
            println!("capture: {:?}", capture);
            // println!("capture len : {:?}", capture.len());

            println!(
                "char: {:?}",
                capture.get(1).unwrap().as_str().chars().next().unwrap()
            );
            println!("text : {:?}", capture.get(0).unwrap().as_str());

            println!(
                "count: {:?}",
                capture.get(0).unwrap().end() - capture.get(0).unwrap().start()
            );
            if curr_count.unwrap() > max_count.unwrap() {
                max_char = curr_char;
                max_count = curr_count;
            }
            println!("{:?}", max_char);
            println!("{:?}", max_count);
        }
    }

    Some((max_char.unwrap(), max_count.unwrap()))

    // Some(cap)
}

fn main() {
    println!("{:?}", longest_repetition(&"abbbbbccccccccccc"))
}

// soln:
use itertools::Itertools;

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    s.chars()
        .dedup_with_count()
        .max_by(|x, y| (x.0 + 1).cmp(&y.0))
        .map(|x| (x.1, x.0))
}

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    s.chars()
        .fold(vec![], |mut result: Vec<Vec<_>>, ch| {
            match result.last_mut().filter(|x| x.contains(&ch)) {
                Some(last) => last.push(ch),
                None => result.push(vec![ch]),
            }

            result
        })
        .iter()
        .rev()
        .max_by_key(|x| x.len())
        .map(|x| (*x.first().unwrap(), x.len()))
}

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s.len() == 0 {
        return None;
    }

    let len = s.len();
    let mut count = 0;
    let mut curr_count = 1;
    let mut res: u8 = 0;

    s.chars().enumerate().for_each(|(i, c)| {
        if i < len - 1 && c == s.chars().nth(i + 1).unwrap() {
            curr_count += 1;
        } else {
            if curr_count > count {
                count = curr_count;
                res = c as u8;
            }
            curr_count = 1;
        }
    });

    Some((res as char, count))
}

use itertools::Itertools;

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    s.chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, g)| (c, g.count()))
        .min_by(|(_, na), (_, nb)| nb.cmp(na))
}

fn longest_repetition(s: &str) -> Option<(char, usize)> {
    if s == "" {
        return None;
    }

    let mut oc = Vec::<(char, usize)>::new();

    let res = s
        .chars()
        .fold((s.chars().next()?, 0), |(acc_char, acc_count), c| {
            return if c == acc_char {
                (c, acc_count + 1)
            } else {
                oc.push((acc_char, acc_count));
                (c, 1)
            };
        });

    oc.push(res);

    Some(*oc.iter().rev().max_by_key(|(c, u)| u)?)
}
