fn change(string: &str) -> String {
    let mut v: Vec<i32> = vec![0; 26];
    let _b = string
        .bytes()
        .map(|x| {
            (if x.is_ascii_alphabetic() {
                v[(x - b'A') as usize] = 1;
                b"abcdefghijklmnopqrstuvwxyz"[(x - b'A') as usize]
            } else {
                x
            }) as char
        })
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")
}
fn main() {
    println!("{:?}", change("a **&  bZ"));
}
