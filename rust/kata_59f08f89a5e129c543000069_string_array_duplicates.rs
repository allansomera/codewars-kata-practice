use regex::Regex;

fn dup(arry: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"(.)+").unwrap();
    for (i, el) in arry.iter().enumerate() {
        let matches = re
            .find_iter(*el)
            .filter_map(|str| str.as_str().parse().ok())
            .collect();
        println!("matches: {:?}", matches);
    }
    todo!()
}

fn main() {
    println!("{:?}", dup(["abracadabra", "allottee", "assessee"]));
}
