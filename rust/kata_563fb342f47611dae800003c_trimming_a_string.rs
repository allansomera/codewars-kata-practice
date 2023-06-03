fn trim(phrase: &str, length: usize) -> String {
    let mut s = String::new();
    if length <= 3 {
        if phrase.len() <= 3 {
            s.push_str(&phrase[0..length].to_string());
        } else {
            s.push_str(&phrase[0..=length - 1].to_string());
            s.push_str("...")
        }
    } else if phrase.len() >= length && phrase.len() >= 3 {
        s.push_str(&phrase[0..(length - 3)].to_string());
        s.push_str("...")
    } else {
        s.push_str(&phrase[0..].to_string());
    }
    s
}

fn main() {
    println!("{:?}", trim("Creating kata is fun", 14));
    println!("{:?}", trim("He", 1));
    println!("{:?}", trim("Hey", 2));
    println!("{:?}", trim("Hey", 3));
    println!("{:?}", trim("Creating kata is fun", 2));
    println!("{:?}", trim("Creating kata is fun", 50));
}
