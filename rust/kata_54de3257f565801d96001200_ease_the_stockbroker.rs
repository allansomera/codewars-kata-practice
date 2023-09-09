fn balance_statement(lst: &str) -> String {
    let vs: Vec<String>;

    if lst.contains(",") {
        vs = lst
            .split(",")
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        println!("comma: {:?}", vs);
    } else {
        vs = lst
            .split_whitespace()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        println!("ws: {:?}", vs);
    }

    "test".to_string()
}

fn main() {
    println!("{:?}", balance_statement("GOOG 300 542.0 B"));
    println!(
        "{:?}",
        balance_statement("GOOG 300 542.0 B,GOOG 300 542.0 B")
    );
}