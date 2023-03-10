fn repeat_str(src: &str, count: usize) -> String {
  let mut z = String::new();
  for  _ in 0..count{
      z.push_str(src)
  }
  return z;
}

fn main(){
    let src = String::from("test");
    let num = 5;
    println!("{}",repeat_str(&src,num));
}

// 1 index
// for _ in 1..=count {
//     ret.push_str(src);
// }

// soln
// fn repeat_str(src: &str, count: usize) -> String {
//   std::iter::repeat(src).take(count).collect()
// }

