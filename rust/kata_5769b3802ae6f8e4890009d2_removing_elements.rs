fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut output = Vec::<u8>::new();
    for (i, el) in arr.iter().enumerate() {
        if i % 2 == 0 {
            output.push(*el)
        }
    }
    output
}

fn main() {
    println!("{:?}", remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8]));
}

// soln:
// fn remove_every_other(arr: &[u8]) -> Vec<u8> {
//     arr.iter().step_by(2).copied().collect()
// }
//
// fn remove_every_other(xs: &[u8]) -> Vec<u8> {
//     xs.iter().step_by(2).cloned().collect()
// }
//
// fn remove_every_other(arr: &[u8]) -> Vec<u8> {
//     arr.iter().step_by(2).map(|x| *x).collect()
// }
//
// fn remove_every_other(arr: &[u8]) -> Vec<u8> {
//     arr.iter().enumerate().filter(|(i, x)| i % 2 == 0).map(|(_,&x)| x).collect::<Vec<_>>()
// }
//
// fn remove_every_other(arr: &[u8]) -> Vec<u8> {
//     let vec: Vec<_> = arr.iter()
//         .enumerate()
//         .filter(|(i, el)| i % 2 == 0 )
//         .map(|(_, el)| *el)
//         .collect();
//
//     return vec
// }
//
// fn remove_every_other(xs: &[u8]) -> Vec<u8> {
//     xs.iter().step_by(2).fold(vec![], |mut acc, n| {
//         acc.push(*n);
//         acc
//     })
// }
//
// fn remove_every_other(arr: &[u8]) -> Vec<u8> {
//     arr.iter()
//         .enumerate()
//         .filter(|(ind, _item)| ind % 2 == 0)
//         .map(|(_ind, item)| item)
//         .copied()
//         .collect()
// }
//
// fn remove_every_other(arr: &[u8]) -> Vec<u8> {
//     (0..)
//         .zip(arr)
//         .filter(|(i, e)| i % 2 == 0)
//         .map(|(i, &e)| e)
//         .collect()
// }
