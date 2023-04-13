fn double_integer(n: i32) -> i32 {
    return n * 2;
}

fn main() {
    // let src = String::from("test");
    // let num = 5;
    let num = double_integer(1);
    println!("{}", num);
}

// solutions:
//
// fn double_integer(n: i32) -> i32 {
//     n<<1
// }
//
// fn double_integer(n: i32) -> i32 {
//     n + n
// }
// fn double_integer(n: i32) -> i32 {
//     let mynum: i32 = n*2;
//     return mynum;
// }
//
// fn double_integer(n: i32) -> i32 {
//     // be fast
//     let x: i32 = &n + &n;
//     x
// }

// no need for a semi-colon because having no semi-colon eveluates to a value,
// a semi-colon means it is an expression
