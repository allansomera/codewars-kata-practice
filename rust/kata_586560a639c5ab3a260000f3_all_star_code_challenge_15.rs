fn rotate(s: &str) -> Vec<String> {
    // let first = s.chars().next().unwrap();
    // let mut vv: Vec<String> = vec![s[1..s.len()].to_string(); s.len() as usize]
    //     .iter()
    //     .map(|x| x.chars().collect::<String>())
    //     .collect::<Vec<_>>();
    // for (idx, x) in vv.iter_mut().enumerate() {
    //     x.insert(idx, first);
    // }

    // vv.iter_mut().for_each(|x| {
    //     x.chars().rev().collect::<String>();
    // });
    // vv.iter_mut().for_each(drop::<&mut String>);
    // vv.reverse();
    // vv
    if s.len() == 0 {
        return vec![];
    }
    let mut word: String = String::from(s);
    // println!("{}", word);
    let mut vv: Vec<String> = Vec::new();
    for _ in 0..s.len() {
        let mut vc: Vec<char> = word.chars().collect::<Vec<_>>();
        let w = vc.remove(0);
        vc.insert(s.len() - 1, w);
        word = vc.into_iter().collect::<String>();
        vv.push(word.clone());
    }
    vv
}

fn main() {
    println!("{:?}", rotate("Hello"));
}

// soln:
// fn rotate(s: &str) -> Vec<String> {
//     (1..=s.len())
//         .map(|i| format!("{}{}", &s[i..], &s[..i]))
//         .collect()
// }
//
// fn rotate(s: &str) -> Vec<String> {
//     let mut result = Vec::new();
//     let mut s = s.to_string();
//     for _ in 0..s.len() {
//         s = s.chars().skip(1).chain(s.chars().take(1)).collect();
//         result.push(s.clone());
//     }
//     result
// }
//
// fn rotate(s: &str) -> Vec<String> {
//     (0..s.len())
//         .map(|i| format!("{}{}",&s[(i+1)..s.len()],&s[0..=i]))
//         .collect()
// }
//
// fn rotate(s: &str) -> Vec<String> {
//     (1..s.len()+1).map(|i| s[i..].to_string()+&s[..i]).collect()
// }
//
// fn rotate(s: &str) -> Vec<String> {
//     let mut result = Vec::<String>::new();
//     let string = String::from(s);
//     for i in 1..=string.len() {
//         let splited = string.split_at(i);
//         result.push(String::from(splited.1) + splited.0);
//     }
//     return result;
// }
//
// fn rotate(s: &str) -> Vec<String> {
//     let len = s.len();
//     let mut it = s.chars().cycle();
//     let mut out = Vec::new();
//     for _ in 0..len {
//         it.next();
//         let mut tmp = String::new();
//         for _ in 0..len {
//             tmp.push(it.next().unwrap().clone())
//         };
//         out.push(tmp);
//     }
//     out
// }
//
// fn rotate(s: &str) -> Vec<String> {
//     let mut inp = s.as_bytes().to_vec();
//     let mut res = Vec::new();
//     for _n in 0..inp.len() {
//         inp.rotate_left(1);
//         res.push(String::from_utf8(inp.clone()).unwrap());
//     }
//     res
// }
