fn number_to_string(i: i32) -> String {
    i.to_string().to_owned()
}

fn main() {
    println!("{:?}", number_to_string(-100));
}

// soln:
// fn number_to_string(i: i32) -> String {
//     let my_string: String = i.to_string();
//     my_string
// }
