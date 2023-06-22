fn get_middle(s: &str) -> &str {
    // let mut pos: usize = 0;
    if s.len() == 2 || s.len() == 1 {
        return s;
    } else if s.len() % 2 == 1 {
        let pos = s.len() / 2;
        // print!("odd\n");
        // println!("pos is {}", pos);
        return &s[pos..pos + 1];
    } else {
        let pos = (s.len() / 2) - 1;
        // print!("even\n");
        // println!("pos is {}", pos);
        return &s[pos..=pos + 1];
    }
}

fn main() {
    println!("{:?}", get_middle("test"));
    println!("{:?}", get_middle("testing"));
    println!("{:?}", get_middle("middle"));
    println!("{:?}", get_middle("A"));
    println!("{:?}", get_middle("of"));
}

// soln:
// fn get_middle(s:&str) -> &str {
//     &s[(s.len()-1)/2..s.len()/2+1]
// }
//
// fn get_middle(s: &str) -> &str {
//   let len = s.len();
//   let from = (len - 1) / 2;
//   let to = (len / 2) + 1;
//   &s[from .. to]
// }
//
// fn get_middle(s: &str) -> &str {
//     let even = s.len() % 2 == 0;
//     let middle = s.len() / 2;
//
//     &s[if even { middle - 1 } else { middle }..middle + 1]
// }
//
// fn get_middle(s:&str) -> &str {
//     match s.len() {
//         0|1|2 => s,
//         len   => &s[(len-1)/2..(len+2)/2]
//     }
// }
//
// fn get_middle(s:&str) -> &str {
//     match s.len() {
//       1 | 2 => &s,
//       _     => get_middle(&s[1..(s.len() - 1)]),
//     }
// }
//
// fn get_middle(s:&str) -> &str {
//     match s.len() {
//       1 => s,
//       2 => s,
//       _ => get_middle(&s[1..s.len()-1])
//     }
// }
//
// fn get_middle(s:&str) -> &str {
//     let l = s.chars().count();
//     let a = (l - 1) / 2;
//     let b = a + 2 - (l % 2);
//     &s[a..b]
// }
//
// fn get_middle(s:&str) -> &str {
//     &s[(s.len()/2+(s.len()&1)-1)..(s.len()/2+1)]
// }
//
// fn get_middle(s:&str) -> &str {
//     let len = s.len();
//     return if len%2==0 {
//         &s[len/2-1..len/2+1]
//     } else {
//         &s[len/2..len/2+1]
//     }
// }
//
// fn get_middle(s:&str) -> &str
// {
//     let mid=(s.len()-1)/2;
//     &s[mid..mid + if s.len()%2==1 {1} else {2}]
// }
