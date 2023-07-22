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
    s
}

fn main() {
    // println!("{:?}", ascend_descend(5, 1, 3));
    // println!("{:?}", ascend_descend(15, 9, 15));
    println!("{:?}", ascend_descend(10, 1, 1));
}
