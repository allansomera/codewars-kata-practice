// use std::any::type_name;

use std::collections::HashMap;
// fn type_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }

fn build(vv: &Vec<String>, multi: bool) -> HashMap<String, HashMap<String, i32>> {
    let mut data: HashMap<String, HashMap<String, i32>> = HashMap::new();
    // let stock_info: HashMap<String, i32> = HashMap::new();

    // ("GOOG 300 542.0 B"));

    if multi {
        println!("multi({}) => {:?}", multi, vv);
        for i in vv.into_iter() {
            let tmp_vv: Vec<String> = i
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            println!("tmp_vv {:?}", tmp_vv);
            println!("check_order() {:?}", check_order(&tmp_vv).join(" "));

            // check_order(i)
            // println!(
            //     "bad order {:?}",
            //     check_order(i.trim().to_string()).join(" ")
            // );
        }
    } else {
        println!("multi ({})", multi.to_string());
        let status = match vv[3].as_ref() {
            "B" => "Buy".to_string(),
            "S" => "Sell".to_string(),
            _ => "".to_string(),
        };
        println!("bad order {:?}", check_order(vv).join(" "));
        println!("{:?}", vv[3]);
        if vv[1].parse::<i32>().is_ok() && vv[2].parse::<f64>().is_ok() {
            let status = "Buy".to_string();
            let quantity = vv[1].clone();
            let price = vv[2].clone();

            data.insert(
                String::from(&vv[0]),
                HashMap::from([(
                    status,
                    vv[1].to_string().parse::<i32>().unwrap()
                        * vv[2].to_string().parse::<f32>().unwrap() as i32,
                    // 1,
                )]),
            );
        } else {
            data.insert(
                String::from(&vv[0]),
                HashMap::from([("Buy".to_string(), 0), ("Sell".to_string(), 0)]),
            );
        }
    }
    // println!("vv[1]: {}", vv[1]);
    // println!("vv[2]: {}", vv[2]);
    data
}

fn check_order(vv: &Vec<String>) -> Vec<String> {
    // let bad_formed: Vec<String> = Vec::new();

    // let mut bad_order: Vec<String> = Vec::new();
    // if vv[1].parse::<f64>().is_ok() {
    //     bad_order = vv.clone();
    // }
    println!(
        "vv[1]_check_float => {:?}: {:?}",
        vv[1],
        vv[1].parse::<i32>().is_ok()
    );

    //if vv[1] == "300.0" will fail
    if !vv[1].parse::<i32>().is_ok() {
        println!("{:?} is a float", vv[1]);
        return vv.clone();
    }
    vv.clone()
}

fn balance_statement(lst: &str) -> String {
    let vs: Vec<String>;

    //multi
    if lst.contains(",") {
        vs = lst
            .split(",")
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        println!("comma: {:?}", vs);
        println!("build {:?}", build(&vs, true));
    } else {
        //non-multi
        vs = lst
            .split_whitespace()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        println!("ws: {:?}", vs);
        println!("build {:?}", build(&vs, false));
        // vs.iter().for_each(|x| println!("{:?}", x));
    }

    // println!("first: {}", vs[0]);
    //
    // for x in vs.iter() {
    //     println!("type_of: {:?}", type_of(&x))
    // }
    // println!("build {:?}", build(&vs, false));
    "test".to_string()
}

fn main() {
    println!(
        "{:?}",
        balance_statement("GOOG 542.0 300 B, GOOG 3000 542.0 S")
    );
    // println!("{:?}", balance_statement("GOOG 542.0 300 B"));
    // println!("{:?}", balance_statement("GOOG 542.0 300 B"));
    // println!(
    //     "{:?}",
    //     balance_statement("GOOG 300 542.0 B,GOOG 300 542.0 B")
    // );
}
