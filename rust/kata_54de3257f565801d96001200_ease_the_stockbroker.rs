// use std::any::type_name;

use std::collections::HashMap;
// fn type_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }

fn build(vv: &Vec<String>, multi: bool) -> HashMap<String, HashMap<String, i32>> {
    let mut data: HashMap<String, HashMap<String, i32>> = HashMap::new();
    // let stock_info: HashMap<String, i32> = HashMap::new();

    if multi {
        todo!()
    } else {
        data.insert(
            String::from(&vv[0]),
            HashMap::from([
                (
                    "Buy".to_string(),
                    // &vv[1].parse::<i32>().unwrap() * &vv[2].parse::<i32>().unwrap(),
                    1,
                ),
                ("Sell".to_string(), 2),
            ]),
        );
    }
    println!("{}", vv[1]);
    println!("{}", vv[2]);
    data
}

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
        vs.iter().for_each(|x| println!("{:?}", x));
    }

    // println!("first: {}", vs[0]);
    //
    // for x in vs.iter() {
    //     println!("type_of: {:?}", type_of(&x))
    // }
    println!("{:?}", build(&vs, false));
    "test".to_string()
}

fn main() {
    println!("{:?}", balance_statement("GOOG 300 542.0 B"));
    // println!(
    //     "{:?}",
    //     balance_statement("GOOG 300 542.0 B,GOOG 300 542.0 B")
    // );
}
