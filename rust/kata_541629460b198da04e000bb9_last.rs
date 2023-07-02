fn last<T: Clone>(slice: &[T]) -> T {
    slice[slice.len() - 1].to_owned()
}

fn main() {
    println!("{:?}", last(&[1, 2, 3, 4]));
}

// soln:
// fn last<T: Clone>(slice: &[T]) -> T {
//   let l = slice.last();
//   match l {
//     None => panic!("empty"),
//     Some(x) => x.clone(),
//   }
// }
//
// fn last<T: Clone>(slice: &[T]) -> T {
//   slice.last().unwrap().clone()
// }
//
// fn last<T: Clone>(slice: &[T]) -> T {
//   slice.last().cloned().unwrap()
// }
//
// fn last<T: Clone>(slice: &[T]) -> T {
//     let last = &slice[slice.len()-1];
//     last.to_owned()
// }
//
// fn last<T: Clone>(s: &[T]) -> T {
//   s.last().unwrap().clone()
// }
