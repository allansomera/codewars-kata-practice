fn no_space(mut x: String) -> String {
    x.retain(|c| !c.is_whitespace());
    x
}

fn main() {
    let src = String::from("dfdfdf dfdfdf df dffdf dfdf");
    println!("{}", no_space(src));
}

// solutions:
// fn no_space(x : String) -> String{
//   x.replace(" ", "")
// }
//
// fn no_space(x : String) -> String{
//   x.split_whitespace().collect()
// }
//
// fn no_space(x : String) -> String{
//     x.chars().filter(|c| !c.is_whitespace()).collect()
// }
//
// fn no_space(x : String) -> String{
//     x.chars().filter(|c| c != &' ').collect()
// }
//
// fn no_space(mut s: String) -> String {
//     s.retain(|c| c != ' ');
//     s
// }
