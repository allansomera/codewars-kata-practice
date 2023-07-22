fn ascend_descend(length: usize, minimum: i32, maximum: i32) -> String {
    if minimum > maximum {
        return "".to_string();
    } else if length == 0 {
        return "".to_string();
    }

    let mut mode = 1;
    let mut s = String::new();
    let mut curr = minimum;
    if minimum == maximum {
        mode = 2;
    }

    println!("mode: {}", mode);
    for _ in 0..length {
        if s.len() >= length {
            return s[0..length].to_string();
        }
        if curr == maximum && mode != 2 {
            mode = 0;
        } else if curr == minimum && mode != 2 {
            mode = 1;
        }
        match mode {
            1 => {
                println!("{}", curr);
                println!("in mode {}", mode);
                s.push_str(&curr.to_string());
                curr += 1;
            }
            0 => {
                println!("{}", curr);
                println!("in mode {}", mode);
                s.push_str(&curr.to_string());
                curr -= 1;
            }
            2 => {
                println!("in mode {}", mode);
                s.push_str(&curr.to_string());
            }
            _ => (),
        }
    }
    s[0..length].to_string()
}

fn main() {
    println!("{:?}", ascend_descend(5, 1, 3));
    println!("{:?}", ascend_descend(15, 9, 15));
    println!("{:?}", ascend_descend(10, 1, 1));
    println!("{:?}", ascend_descend(1, -5, -4));
}

soln:
fn ascend_descend(l: usize, a: i32, b: i32) -> String {
    (a..=a.max(b - 1)).chain((a + 1..=b).rev()).cycle()
    .flat_map(|x| x.to_string().chars().collect::<Vec<_>>())
    .take(if a > b { 0 } else { l })
    .collect()
}

fn ascend_descend(length: usize, min: i32, max: i32) -> String {
    let ascend: String = (min..=max).map(|x| x.to_string()).collect();
    let descend: String = ((min + 1)..max).rev().map(|x| x.to_string()).collect();
    
    (ascend + &descend).chars().cycle().take(length).collect()
}

fn ascend_descend(length: usize, minimum: i32, maximum: i32) -> String {
    let input = (minimum..=maximum).chain((minimum+1..maximum).rev()).collect::<Vec<i32>>();
    input.iter().cycle().map(|x| x.to_string()).take(length).collect::<String>().get(0..length).unwrap_or("").to_string()
}

fn ascend_descend(length: usize, minimum: i32, maximum: i32) -> String {
    
    (minimum..=maximum)
        .chain((minimum+1..maximum).rev())
        .cycle()
        .map(|i| i.to_string().chars().collect::<Vec<_>>())
        .flatten()
        .take(length)
        .collect::<String>()
}
