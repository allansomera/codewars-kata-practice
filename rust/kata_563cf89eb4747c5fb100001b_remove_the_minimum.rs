fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.len() == 0 {
        return vec![];
    }
    let mut c: Vec<u32> = numbers.to_vec();
    let (small_idx, _) = c
        .iter()
        .enumerate()
        .min_by(|(_, &a), (_, &b)| a.cmp(&b))
        .unwrap();
    c.remove(small_idx);
    c
}

fn main() {
    println!("{:?}", remove_smallest(&[5, 5, 5, 5, 1, 5, 1]));
}

// soln:
// fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
//     if numbers.len()<1 {return vec![];}
//
//     let min = numbers.iter().enumerate()
//     .fold((0, u32::MAX), |a,(i,x)| if a.1 > *x {(i,*x)}else{a});
//
//     let mut r = numbers.to_vec();
//     r.remove(min.0);
//     r
// }
//
// fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
//     if let Some(min) = numbers.iter().min() {
//        if let Some(pos) = numbers.iter().position(|&x| x == *min) {
//            let mut result = Vec::from(numbers);
//            result.remove(pos);
//            return result;
//        }
//     }
//     vec![]
// }
//
// fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
//     let smallest = *numbers.to_vec().iter().min().unwrap_or(&0);
//     let nums = numbers.to_vec();
//     let mut v = vec![];
//     let mut b = true;
//     for item in nums {
//         if item == smallest && b {
//             b = false;
//             continue;
//         }
//         v.push(item)
//     }
//     v
// }
//
// fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
//     let mut res:Vec<u32> = numbers.to_vec();
//     if res.len() == 0 {return res;}
//
//     let min = res.iter().min().unwrap().clone();
//     let lowest = res.iter().position(|&x| x == min);
//     match lowest {
//         Some(x) => {res.remove(x);},
//         None => ()
//     };
//     res
// }
//
// fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
//     let mut num = numbers.to_vec();
//     if num.is_empty(){
//         return num;
//     }
//     num.remove(numbers.to_vec().iter().position(|x| x == num.iter().min().unwrap()).unwrap());
//     num
// }
//
// fn remove_smallest(n: &[u32]) -> Vec<u32> {
//     let mut v = n.to_vec();
//
//     match v.iter().min_by(|x, y| x.cmp(&y)) {
//         Some(x) => {
//             let i = v.iter().position(|&i| i == *x).unwrap();
//             v.remove(i);
//             v
//         }
//         None => Vec::new(),
//     }
// }
