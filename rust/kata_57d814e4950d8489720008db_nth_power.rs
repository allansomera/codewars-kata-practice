fn index(nums: &[u64], n: usize) -> Option<u64> {
    if n > nums.len() {
        return None;
    }
    Some(u64::pow(nums[n], n as u32))
}

fn main() {
    println!("{:?}", index(&[1, 2, 3, 4, 4], 2));
}

// soln:
// fn index(nums: &[u64], n: usize) -> Option<u64> {
//     nums.get(n).map(|x| x.pow(n as u32))
// }
//
// fn index(nums: &[u64], n: usize) -> Option<u64> {
//     match nums.get(n) {
//         Some(i) => { return Some(i.pow(n as u32)); }
//         None => { return None; }
//     }
// }
//
// fn index(nums: &[u64], n: usize) -> Option<u64> {
//     nums.get(n).and_then(|x| Some(x.pow(n as u32)))
// }
//
// fn index(nums: &[u64], n: usize) -> Option<u64> {
//     nums.get(n).map(|&i| i.pow(n as u32))
// }
//
// fn index(nums: &[u64], n: usize) -> Option<u64> {
//     if n<0 || n>= nums.len() {
//         return None
//     }
//     Some(u64::pow(nums[n], n.try_into().unwrap()))
// }
//
// fn index(nums: &[u64], n: usize) -> Option<u64> {
//     if n >= nums.len() {return None;}
//
//     Some((nums[n] as u64).pow(n as u32))
// }
