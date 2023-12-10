fn dup(arry: Vec<String>) -> Vec<String> {
    let mut n_vec: Vec<String> = Vec::new();
    for s in &arry {
        let mut result = String::new();
        let mut prev_char: Option<char> = None;
        for c in s.chars() {
            if prev_char != Some(c) {
                result.push(c);
            }
            prev_char = Some(c);
        }
        n_vec.push(result.clone());
    }
    todo!()
}

fn main() {
    println!(
        "{:?}",
        dup(vec![
            "abracadabra".to_string(),
            "allottee".to_string(),
            "assessee".to_string()
        ])
    );
}
