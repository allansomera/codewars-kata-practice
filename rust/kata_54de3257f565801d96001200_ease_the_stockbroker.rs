// use std::any::type_name;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
// fn type_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }
// use std::error;
//
// type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn build(vv: &Vec<String>, multi: bool) -> (HashMap<String, HashMap<String, f64>>, Vec<String>) {
    let mut data: HashMap<String, HashMap<String, f64>> = HashMap::new();
    // let mut bad_order: String = String::new();
    let mut bad_orders: Vec<String> = Vec::new();
    // let buy_sell: Vec<Vec<(String, i32)>> = Vec::new();
    // let stock_info: HashMap<String, i32> = HashMap::new();

    // ("GOOG 300 542.0 B"));

    if multi {
        println!("multi({}) => {:?}", multi, vv);
        println!("\ninside for: {}", "\n=====\n");
        for i in vv.into_iter() {
            let tmp_vv: Vec<String> = i
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            println!("tmp_vv {:?}", tmp_vv);
            // println!("check_order() {:?}", check_order(&tmp_vv).join(" "));

            // check_order(i)
            // println!(
            //     "bad order {:?}",
            //     check_order(i.trim().to_string()).join(" ")
            // );
            // println!("check_order: {:?}", check_order(&tmp_vv));
            // let build_data = match check_order(&tmp_vv) {
            //     //build data
            //     Ok(o_data) => {
            //         let status = match o_data[3].as_ref() {
            //             "B" => "Buy".to_string(),
            //             "S" => "Sell".to_string(),
            //             &_ => "".to_string(),
            //         };
            //         println!("status {:?}", status);
            //         // let v_buysell: Vec<(String, i32)> = Vec::new();
            //         println!("contains_key {:?}", data.contains_key(&vv[0]));
            //         // v_buysell.insert((status, 0));
            //         // data.insert(
            //         //     String::from(&o[0]),
            //         //     HashMap::from([(
            //         //         status,
            //         //         o[1].to_string().parse::<i32>().unwrap()
            //         //             * o[2].to_string().parse::<f32>().unwrap() as i32,
            //         //     )]),
            //         // );
            //         let result = match data.entry(o_data[0].clone()) {
            //             // check for stock name like GOOG if it exist
            //             Vacant(new_stock_name) => {
            //                 println!("{}", "new_stock_name vacant".to_string());
            //                 // let stock_key = new_stock_name.get().clone();
            //                 // n.insert(HashMap::from([(
            //                 //     status,
            //                 //     o[1].to_string().parse::<i32>().unwrap()
            //                 //         * o[2].to_string().parse::<f32>().unwrap() as i32,
            //                 // )]));
            //                 // n.insert(HashMap::from([(
            //                 //     status,
            //                 //     o_data[1].to_string().parse::<i32>().unwrap()
            //                 //         * o_data[2].to_string().parse::<f32>().unwrap() as i32,
            //                 // )]));
            //                 let mut buy_sell: HashMap<String, i32> = HashMap::new();
            //
            //                 // since the hashmap has the structure {GOOG: {Buy: 23123, Sell: 123123}}
            //                 // grab the status and insert a new hashmap
            //                 buy_sell.insert(
            //                     status.clone(),
            //                     o_data[1].to_string().parse::<i32>().unwrap()
            //                         * o_data[2].to_string().parse::<f32>().unwrap() as i32,
            //                 );
            //                 new_stock_name.insert(buy_sell.clone());
            //                 Ok(())
            //             }
            //             // GOOG stock name exist
            //             Occupied(mut occ) => {
            //                 //occ the value of what {GOOG: {}} is holding
            //                 let mut bs_status = occ.get().clone();
            //                 // print_type_of(&bs_status);
            //                 // let occ_keys = occ.key();
            //                 // println!("occ_keys{:?}", occ_keys);
            //
            //                 //check if key 'Buy' or 'Sell' exists
            //                 match bs_status.entry(status.clone()) {
            //                     //if they key doesnt exist
            //                     Vacant(v_buysell) => {
            //                         println!("v_buysell: {:?}", v_buysell);
            //                         // let bs: HashMap<String, i32> = HashMap::new();
            //                         //insert new value => shares * price
            //                         v_buysell.insert(
            //                             o_data[1].to_string().parse::<i32>().unwrap()
            //                                 * o_data[2].to_string().parse::<f32>().unwrap() as i32,
            //                         );
            //                         // println!("v_buysell=>after: {:?}", v_buysell);
            //                         // occ.insert(bs.clone());
            //                         // println!("occ: {:?}", occ)
            //                     }
            //                     //if key 'Buy' or 'Sell' exists
            //                     Occupied(mut buysell_exist) => {
            //                         // let prev = occ.get(status);
            //                         // let prev = occ.get(status.clone());
            //
            //                         //get previous value {Buy: 123} => prev_val == 123
            //                         let prev_val = buysell_exist.get();
            //                         // let prev_key = buysell_exist.key();
            //
            //                         //add new value => share * price + prev_val
            //                         buysell_exist.insert(
            //                             o_data[1].to_string().parse::<i32>().unwrap()
            //                                 * o_data[2].to_string().parse::<f32>().unwrap() as i32
            //                                 + prev_val,
            //                         );
            //                         // println!("prev: {:?}", prev);
            //                         // buysell_exist
            //                         // occ.get_mut().insert(status.clone(),
            //                         //
            //                         //                      )
            //                     }
            //                 }
            //                 println!("bs_status: {:?}", bs_status);
            //                 // occ.get_mut().insert(
            //                 //     status.clone(),
            //                 //     o_data[1].to_string().parse::<i32>().unwrap()
            //                 //         * o_data[2].to_string().parse::<f32>().unwrap() as i32,
            //                 // );
            //
            //                 //bs_status will hold the new added/updated HashMap of {Buy/Sell: value}
            //                 occ.insert(bs_status.clone());
            //                 Err(())
            //                 // println!("Occupied {:?}", value);
            //                 // let mut buy_or_sell;
            //                 // let prev;
            //                 // if it exist check for Buy or Sell and update
            //                 // if let Some(key) = stock_key.keys().next() {
            //                 //     buy_or_sell = key.clone();
            //                 //     prev = stock_key.get(&buy_or_sell);
            //                 //     println!("value: {:?}", prev.unwrap());
            //                 //
            //                 //     // stock_name.insert(HashMap::from([(
            //                 //     //     buy_or_sell,
            //                 //     //     o[1].to_string().parse::<i32>().unwrap()
            //                 //     //         * o[2].to_string().parse::<f32>().unwrap() as i32
            //                 //     //         + prev.unwrap(),
            //                 //     // )]));
            //                 //     // println!("v: {:?}", v)
            //                 // }
            //                 // HashMap::from([(
            //                 //     status,
            //                 //     o[1].to_string().parse::<i32>().unwrap()
            //                 //         * o[2].to_string().parse::<f32>().unwrap() as i32,
            //                 // )]),
            //             }
            //         }; //result
            //         println!("result {:?}", result);
            //         o_data
            //     }
            //     // Err(e) => e,
            //     Err(e) => {
            //         println!("bad_order: {:?}", e);
            //         let bad_order =
            //             String::from(e.clone().into_iter().collect::<Vec<_>>().join(" "));
            //         bad_orders.push(bad_order.clone());
            //         e
            //     }
            // };
            // let val = check_order(&tmp_vv)?;
            // println!("build_data: {:?}", build_data);
            // println!("val - type : {:?}", check_order(&tmp_vv));
            // println!(
            //     "{}",
            //     "\n+++++++++++++++++++++++++++++++++++++++++\n".to_string()
            // );

            parse_data(&tmp_vv, &mut data, &mut bad_orders);
        }
        // (data, bad_orders)
    } else {
        //multi is false

        println!("multi ({})", multi.to_string());
        // let status = match vv[3].as_ref() {
        //     "B" => "Buy".to_string(),
        //     "S" => "Sell".to_string(),
        //     _ => "".to_string(),
        // };
        // // println!("bad order {:?}", check_order(vv).join(" "));
        // println!("{:?}", vv[3]);
        // if vv[1].parse::<i32>().is_ok() && vv[2].parse::<f64>().is_ok() {
        //     let status = "Buy".to_string();
        //     let quantity = vv[1].clone();
        //     let price = vv[2].clone();
        //
        //     data.insert(
        //         String::from(&vv[0]),
        //         HashMap::from([(
        //             status,
        //             vv[1].to_string().parse::<i32>().unwrap()
        //                 * vv[2].to_string().parse::<f32>().unwrap() as i32,
        //             // 1,
        //         )]),
        //     );
        // } else {
        //     data.insert(
        //         String::from(&vv[0]),
        //         HashMap::from([("Buy".to_string(), 0), ("Sell".to_string(), 0)]),
        //     );
        // }

        // check if correct format
        // let tmp_vv_single: Vec<String> = vv
        //     .clone()
        //     .split_whitespace()
        //     .into_iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>();
        // println!("tmp_vv_single => {:?}", tmp_vv_single);
        println!("vv => {:?}", vv);
        // let build_data = match check_order(&vv) {
        //     Ok(o_data_s) => {
        //         let status = match o_data_s[3].as_ref() {
        //             "B" => "Buy".to_string(),
        //             "S" => "Sell".to_string(),
        //             &_ => "".to_string(),
        //         };
        //
        //         let result = match data.entry(o_data_s[0].clone()) {
        //             // check for stock name like GOOG if it exist
        //             Vacant(new_stock_name) => {
        //                 println!("{}", "new_stock_name vacant".to_string());
        //                 let mut buy_sell: HashMap<String, i32> = HashMap::new();
        //
        //                 // since the hashmap has the structure {GOOG: {Buy: 23123, Sell: 123123}}
        //                 // grab the status and insert a new hashmap
        //                 buy_sell.insert(
        //                     status.clone(),
        //                     o_data_s[1].to_string().parse::<i32>().unwrap()
        //                         * o_data_s[2].to_string().parse::<f32>().unwrap() as i32,
        //                 );
        //                 new_stock_name.insert(buy_sell.clone());
        //                 Ok(())
        //             }
        //             // GOOG stock name exist
        //             Occupied(mut occ) => {
        //                 //occ the value of what {GOOG: {}} is holding
        //                 let mut bs_status = occ.get().clone();
        //
        //                 //check if key 'Buy' or 'Sell' exists
        //                 match bs_status.entry(status.clone()) {
        //                     //if they key doesnt exist
        //                     Vacant(v_buysell) => {
        //                         println!("v_buysell: {:?}", v_buysell);
        //                         // let bs: HashMap<String, i32> = HashMap::new();
        //                         //insert new value => shares * price
        //                         v_buysell.insert(
        //                             o_data_s[1].to_string().parse::<i32>().unwrap()
        //                                 * o_data_s[2].to_string().parse::<f32>().unwrap() as i32,
        //                         );
        //                     }
        //                     //if key 'Buy' or 'Sell' exists
        //                     Occupied(mut buysell_exist) => {
        //                         //get previous value {Buy: 123} => prev_val == 123
        //                         let prev_val = buysell_exist.get();
        //                         // let prev_key = buysell_exist.key();
        //
        //                         //add new value => share * price + prev_val
        //                         buysell_exist.insert(
        //                             o_data_s[1].to_string().parse::<i32>().unwrap()
        //                                 * o_data_s[2].to_string().parse::<f32>().unwrap() as i32
        //                                 + prev_val,
        //                         );
        //                     }
        //                 }
        //                 println!("bs_status: {:?}", bs_status);
        //                 occ.insert(bs_status.clone());
        //                 Err(())
        //             }
        //         }; //result
        //         o_data_s
        //     }
        //
        //     Err(e_data_s) => {
        //         println!("bad_order: {:?}", e_data_s);
        //         let bad_order =
        //             String::from(e_data_s.clone().into_iter().collect::<Vec<_>>().join(" "));
        //         bad_orders.push(bad_order.clone());
        //         e_data_s
        //     } // add to correct Buy Sell hashmaps
        //
        //       // build data
        // };
        // println!("vv[1]: {}", vv[1]);
        // println!("vv[2]: {}", vv[2]);
        // println!("data {:?}", data);
        // (data, bad_orders) = parse_data(&vv)
        parse_data(&vv, &mut data, &mut bad_orders)
    }
    for (key, value) in &mut data {
        println!("[BEFORE] Key: {}, Value: {:?}", key, value);
        println!("Value_keys => {:?}", value.keys());
        if value.keys().len() == 1 {
            let temp_status = value.keys().next().unwrap();
            match temp_status.as_ref() {
                "Buy" => value.insert(
                    "Sell".to_string(),
                    format!("{:.0}", 0).parse::<f64>().unwrap(),
                ),
                "Sell" => value.insert(
                    "Buy".to_string(),
                    format!("{:.0}", 0).parse::<f64>().unwrap(),
                ),
                &_ => None,
            };
            // println!("temp_status {:?}", temp_status);
        }

        // value.insert("test".to_string(), 1);
        println!("[AFTER] Key: {}, Value: {:?}", key, value);
    }
    (data.clone(), bad_orders.clone())
}

fn check_order(vv: &Vec<String>) -> Result<Vec<String>, Vec<String>> {
    // let bad_formed: Vec<String> = Vec::new();

    // let mut bad_order: Vec<String> = Vec::new();
    // if vv[1].parse::<f64>().is_ok() {
    //     bad_order = vv.clone();
    // }
    println!("check_order() vv => {:?}", vv);
    println!(
        "vv[1]_check_float => {:?}: {:?}",
        vv[0],
        vv[1].parse::<i32>().is_ok()
    );
    let order = vv[1].parse::<i32>().is_ok();
    let price = !vv[2].parse::<i32>().is_ok();
    let vv_length_true = vv.len() == 4;
    let letter: bool;
    if vv_length_true {
        letter = match vv[3].as_ref() {
            "B" | "S" => true,
            _ => false,
        };
    } else {
        letter = false;
    }

    println!("order is i32 {:?}", order);
    println!("price is f64 {:?}", price);

    //if vv[1] == "300.0" will fail
    // if !vv[1].parse::<i32>().is_ok() {
    //     println!("{:?} is a float", vv[1]);
    //     return Err(vv.clone());
    // } else {
    //     return Ok(vv.clone());
    // }
    if ((order && price) && vv_length_true) && letter {
        Ok(vv.clone())
    } else {
        Err(vv.clone())
    }
}

fn parse_data(
    tmp_vv: &Vec<String>,
    data: &mut HashMap<String, HashMap<String, f64>>,
    bad_orders: &mut Vec<String>,
    // ) -> (HashMap<String, HashMap<String, f64>>, Vec<String>) {
) {
    // let mut data: HashMap<String, HashMap<String, f64>> = HashMap::new();
    // let mut bad_order: String = String::new();
    // let mut bad_orders: Vec<String> = Vec::new();

    let _build_data = match check_order(&tmp_vv) {
        //build data
        Ok(o_data) => {
            let status = match o_data[3].as_ref() {
                "B" => "Buy".to_string(),
                "S" => "Sell".to_string(),
                &_ => "".to_string(),
            };
            // println!("status {:?}", status);
            // let v_buysell: Vec<(String, i32)> = Vec::new();
            // println!("contains_key {:?}", data.contains_key(&vv[0]));
            // v_buysell.insert((status, 0));
            // data.insert(
            //     String::from(&o[0]),
            //     HashMap::from([(
            //         status,
            //         o[1].to_string().parse::<i32>().unwrap()
            //             * o[2].to_string().parse::<f32>().unwrap() as i32,
            //     )]),
            // );
            let result = match data.entry(o_data[0].clone()) {
                // check for stock name like GOOG if it exist
                Vacant(new_stock_name) => {
                    println!("{}", "new_stock_name vacant".to_string());
                    // let stock_key = new_stock_name.get().clone();
                    // n.insert(HashMap::from([(
                    //     status,
                    //     o[1].to_string().parse::<i32>().unwrap()
                    //         * o[2].to_string().parse::<f32>().unwrap() as i32,
                    // )]));
                    // n.insert(HashMap::from([(
                    //     status,
                    //     o_data[1].to_string().parse::<i32>().unwrap()
                    //         * o_data[2].to_string().parse::<f32>().unwrap() as i32,
                    // )]));
                    let mut buy_sell: HashMap<String, f64> = HashMap::new();

                    // since the hashmap has the structure {GOOG: {Buy: 23123, Sell: 123123}}
                    // grab the status and insert a new hashmap
                    buy_sell.insert(
                        status.clone(),
                        ((o_data[1].to_string().parse::<f64>().unwrap() as f64)
                            * o_data[2].to_string().parse::<f64>().unwrap())
                            as f64,
                    );
                    new_stock_name.insert(buy_sell.clone());
                    Ok(())
                }
                // GOOG stock name exist
                Occupied(mut occ) => {
                    //occ the value of what {GOOG: {}} is holding
                    let mut bs_status = occ.get().clone();
                    // print_type_of(&bs_status);
                    // let occ_keys = occ.key();
                    // println!("occ_keys{:?}", occ_keys);

                    //check if key 'Buy' or 'Sell' exists
                    match bs_status.entry(status.clone()) {
                        //if they key doesnt exist
                        Vacant(v_buysell) => {
                            println!("v_buysell: {:?}", v_buysell);
                            // let bs: HashMap<String, i32> = HashMap::new();
                            //insert new value => shares * price
                            v_buysell.insert(
                                (o_data[1].to_string().parse::<f64>().unwrap() as f64
                                    * o_data[2].to_string().parse::<f64>().unwrap())
                                    as f64,
                            );
                            // println!("v_buysell=>after: {:?}", v_buysell);
                            // occ.insert(bs.clone());
                            // println!("occ: {:?}", occ)
                        }
                        //if key 'Buy' or 'Sell' exists
                        Occupied(mut buysell_exist) => {
                            // let prev = occ.get(status);
                            // let prev = occ.get(status.clone());

                            //get previous value {Buy: 123} => prev_val == 123
                            let prev_val = buysell_exist.get();
                            // let prev_key = buysell_exist.key();

                            //add new value => share * price + prev_val
                            buysell_exist.insert(
                                (o_data[1].to_string().parse::<i32>().unwrap() as f64
                                    * o_data[2].to_string().parse::<f64>().unwrap())
                                    as f64
                                    + *prev_val as f64,
                            );
                            // println!("prev: {:?}", prev);
                            // buysell_exist
                            // occ.get_mut().insert(status.clone(),
                            //
                            //                      )
                        }
                    }
                    println!("bs_status: {:?}", bs_status);
                    // occ.get_mut().insert(
                    //     status.clone(),
                    //     o_data[1].to_string().parse::<i32>().unwrap()
                    //         * o_data[2].to_string().parse::<f32>().unwrap() as i32,
                    // );

                    //bs_status will hold the new added/updated HashMap of {Buy/Sell: value}
                    occ.insert(bs_status.clone());
                    Err(())
                    // println!("Occupied {:?}", value);
                    // let mut buy_or_sell;
                    // let prev;
                    // if it exist check for Buy or Sell and update
                    // if let Some(key) = stock_key.keys().next() {
                    //     buy_or_sell = key.clone();
                    //     prev = stock_key.get(&buy_or_sell);
                    //     println!("value: {:?}", prev.unwrap());
                    //
                    //     // stock_name.insert(HashMap::from([(
                    //     //     buy_or_sell,
                    //     //     o[1].to_string().parse::<i32>().unwrap()
                    //     //         * o[2].to_string().parse::<f32>().unwrap() as i32
                    //     //         + prev.unwrap(),
                    //     // )]));
                    //     // println!("v: {:?}", v)
                    // }
                    // HashMap::from([(
                    //     status,
                    //     o[1].to_string().parse::<i32>().unwrap()
                    //         * o[2].to_string().parse::<f32>().unwrap() as i32,
                    // )]),
                }
            }; //result
            println!("result {:?}", result);
            o_data
        }
        // Err(e) => e,
        Err(e) => {
            println!("bad_order: {:?}", e);
            let bad_order = String::from(e.clone().into_iter().collect::<Vec<_>>().join(" "));
            bad_orders.push(bad_order.clone());
            println!("bad_orders Err(e) {:?}", bad_orders);
            e
        }
    };

    // println!("build_data: {:?}", build_data);
    println!("val - type : {:?}", check_order(&tmp_vv));
    println!(
        "{}",
        "\n+++++++++++++++++++++++++++++++++++++++++\n".to_string()
    );

    for (key, mut value) in data.clone().into_iter() {
        // println!("[BEFORE] Key: {}, Value: {:?}", key, value);
        // println!("Value_keys => {:?}", value.keys());

        if value.keys().len() == 1 {
            let temp_status = value.keys().next().unwrap();
            match temp_status.as_ref() {
                "Buy" => value.insert(
                    "Sell".to_string(),
                    format!("{:.0}", 0).parse::<f64>().unwrap(),
                ),
                "Sell" => value.insert(
                    "Buy".to_string(),
                    format!("{:.0}", 0).parse::<f64>().unwrap(),
                ),
                &_ => None,
            };
            // println!("temp_status {:?}", temp_status);
        }

        // value.insert("test".to_string(), 1);
        println!("[AFTER] Key: {}, Value: {:?}", key, value);
    }
    println!("parse data {:?}", data.clone());
    println!("parse bad_orders {:?}", bad_orders);
    // (data.clone(), bad_orders.clone())
}

fn banker_round(x: &f64) -> i64 {
    let whole = x.clone().floor() as i64;
    let fraction = x - whole as f64;

    if fraction < 0.5 {
        whole
    } else if fraction > 0.5 {
        whole + 1
    } else {
        if whole % 2 == 0 {
            whole
        } else {
            whole + 1
        }
    }
}

fn format_statement(
    final_statement: &mut String,
    bad_orders_string: &mut String,
    bad_orders: &Vec<String>,
    buy: &f64,
    sell: &f64,
) {
    let buy_rounded: i64;
    let sell_rounded: i64;
    *bad_orders_string = match bad_orders.len() {
        0 => "".to_string(),
        _ => bad_orders
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ;"),
    };
    buy_rounded = banker_round(buy);
    sell_rounded = banker_round(sell);

    *final_statement = match bad_orders.len() {
        0 => format!("Buy : {:?} Sell: {:?}", buy_rounded, sell_rounded,),
        _ => format!(
            "Buy : {:?} Sell: {:?}; Badly formed {}: {} ;",
            buy_rounded,
            sell_rounded,
            bad_orders.len(),
            bad_orders_string
        ),
    };
    // println!(
    //     "format_statement bad_orders_string: {:?}",
    //     bad_orders_string
    // );
    // println!("format_statement bad_orders: {:?}", bad_orders);
    // println!("format_statement bad_orders_string {:?}", bad_orders_string)
}
fn balance_statement(lst: &str) -> String {
    let vs: Vec<String>;

    let mut final_statement: String = String::new();
    let mut bad_orders_string: String = String::new();
    let mut buy = 0_f64;
    let mut sell = 0_f64;
    //multi
    if lst.contains(",") {
        vs = lst
            .split(",")
            .into_iter()
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();
        println!("comma: {:?}", vs);
        // println!("build {:?}", build(&vs, true));
        let (good_orders, bad_orders) = build(&vs, true);
        println!("good_orders: {:?}", good_orders);
        println!("bad_orders: {:?}", bad_orders);
        println!("Badly formed {}", bad_orders.len());
        for (_key, value) in &good_orders {
            for (v_key, v_val) in value {
                println!("v_key: {:?}, v_val: {:?}", v_key, v_val);
                match v_key.as_ref() {
                    "Buy" => buy += v_val,
                    "Sell" => sell += v_val,
                    &_ => (),
                }
            }
            // println!("Key: {:?}, Value: {:?}", key, value)
        }

        // if bad_orders.len() == 0 {
        //     bad_orders_string = "".to_string()
        // } else {
        //     bad_orders_string = bad_orders
        //         .clone()
        //         .into_iter()
        //         .map(|x| x.to_string())
        //         .collect::<Vec<String>>()
        //         .join(" ;");
        // }

        // bad_orders_string = match bad_orders.len() {
        //     0 => "".to_string(),
        //     _ => bad_orders
        //         .clone()
        //         .into_iter()
        //         .map(|x| x.to_string())
        //         .collect::<Vec<String>>()
        //         .join(" ;"),
        // };

        // final_statement = format!(
        //     "Buy : {:?} Sell: {:?}; Badly formed {}: {} ;",
        //     buy as i64,
        //     sell as i64,
        //     bad_orders.len(),
        //     bad_orders_string
        // );

        // final_statement = match bad_orders.len() {
        //     0 => format!("Buy : {:?} Sell: {:?};", buy as i64, sell as i64,),
        //     _ => format!(
        //         "Buy : {:?} Sell: {:?}; Badly formed {}: {} ;",
        //         buy.round() as i64,
        //         sell.round() as i64,
        //         bad_orders.len(),
        //         bad_orders_string
        //     ),
        // };

        format_statement(
            &mut final_statement,
            &mut bad_orders_string,
            &bad_orders,
            &buy,
            &sell,
        );
        // println!("[AFTER] bad_orders_string {:?}", bad_orders_string);
        println!("buy: {}", buy);
        println!("sell: {}", sell);
        return final_statement;
    } else {
        //non-multi
        vs = lst
            .split_whitespace()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        println!("ws: {:?}", vs);
        let (good_orders, bad_orders) = build(&vs, false);
        println!("build {:?}", good_orders);
        // vs.iter().for_each(|x| println!("{:?}", x));
        // let mut final_statement: String = String::new();
        // let mut bad_orders_string: String = String::new();
        // let mut buy = 0_f64;
        // let mut sell = 0_f64;
        // bad_orders_string = match bad_orders.len() {
        //     0 => "".to_string(),
        //     _ => bad_orders
        //         .clone()
        //         .into_iter()
        //         .map(|x| x.to_string())
        //         .collect::<Vec<String>>()
        //         .join(" ;"),
        // };
        for (_key, value) in &good_orders {
            for (v_key, v_val) in value {
                println!("v_key: {:?}, v_val: {:?}", v_key, v_val);
                match v_key.as_ref() {
                    "Buy" => buy += v_val,
                    "Sell" => sell += v_val,
                    &_ => (),
                }
            }
            // println!("Key: {:?}, Value: {:?}", key, value)
        }

        // final_statement = match bad_orders.len() {
        //     0 => format!("Buy : {:?} Sell: {:?};", buy as i64, sell as i64,),
        //     _ => format!(
        //         "Buy : {:?} Sell: {:?}; Badly formed {}: {} ;",
        //         buy.round() as i64,
        //         sell.round() as i64,
        //         bad_orders.len(),
        //         bad_orders_string
        //     ),
        // };
        format_statement(
            &mut final_statement,
            &mut bad_orders_string,
            &bad_orders,
            &buy,
            &sell,
        );
        return final_statement;
    }

    // println!("first: {}", vs[0]);
    //
    // for x in vs.iter() {
    //     println!("type_of: {:?}", type_of(&x))
    // }
    // println!("build {:?}", build(&vs, false));
}

fn main() {
    // println!(
    //     "{:?}",
    //     balance_statement("GOOG 542.0 300 B, GOOG 3000 542.0 S")
    // );
    // println!(
    //     "{:?}",
    //     balance_statement("GOOG 542.0 300 B, APPL 3000 542.0 S, GOOG 300 542.0 B")
    // );
    // println!(
    //     "{:?}",
    //     balance_statement("GOOG 542.0 300 B, APPL 100 10.0 S, GOOG 100 10.0 B")
    // );
    // println!("{:?}", balance_statement("GOOG 300 542.0 B, APPL 200.0 20"));
    // println!("{:?}", balance_statement("GOOG 300 542.0 B"));
    // println!("{:?}", balance_statement("GOOG 1 542.0 B"));
    // println!("{:?}", balance_statement("GOOG 542.0 300 B"));
    // println!(
    //     "{:?}",
    //     balance_statement(
    //         "GOOG 1 10.0 B,GOOG 1 10.0 B,GOOG 1 10.0 B,GOOG 1 10.0 S, APPL 10.0 1 B, APPL 10.0 1 B, APPL 2 10.0 B"
    //     ));
    // balance_statement("ZNGA 1300 2.66 B, CLH15.NYM 50 56.32 B, OWW 1000 11.623 B, OGG 20 580.1 B");
    // );
    // println!(
    //     "{:?}",
    //     balance_statement("GOOG 300 542.0 B, AAPL 50 145.0 B, CSCO 250.0 29 B, GOOG 200 580.0 S")
    // );
    println!(
        "{:?}",
        balance_statement("JPMC 24.0 55.5 S, MY SPACE 50 45.5 B, CITI 67 5.5 B, CSCO 45 210 S, GOOG 15 160.45 P, ZYGN 100 89.3 C")
    );
}

// soln:
// fn balance_statement(lst: &str) -> String {
//     if lst.is_empty() {
//         return String::from("Buy: 0 Sell: 0");
//     }
//
//     let mut buy = 0.0;
//     let mut sell = 0.0;
//     let mut malformed = vec![];
//
//     for order in lst.split(", ") {
//         if let [quote, quantity, price, status] =
//             order.split_whitespace().collect::<Vec<_>>().as_slice()
//         {
//             if quote.contains(" ") || !price.contains(".") {
//                 malformed.push(order);
//             } else {
//                 match (*status, quantity.parse::<u64>(), price.parse::<f64>()) {
//                     ("B", Ok(q), Ok(p)) => buy += q as f64 * p,
//                     ("S", Ok(q), Ok(p)) => sell += q as f64 * p,
//                     _ => {
//                         malformed.push(order);
//                     }
//                 }
//             }
//         } else {
//             malformed.push(order);
//         }
//     }
//
//     let mut result = format!("Buy: {:.0} Sell: {:.0}", buy, sell);
//
//     if !malformed.is_empty() {
//         result += format!(
//             "; Badly formed {}: {} ;",
//             malformed.len(),
//             malformed.join(" ;")
//         )
//         .as_str();
//     }
//
//     result
// }
//
// #[macro_use] extern crate lazy_static;
// use regex::Regex;
//
// fn balance_statement(lst: &str) -> String {
//     lazy_static! {
//         static ref FMT: Regex = Regex::new(r"^\S+ (\d+) (\d*\.\d*) ([BS])$").unwrap();
//     }
//     let mut buy = 0.0f64;
//     let mut sell = 0.0f64;
//     let mut bad: Vec<&str> = vec![];
//     lst.split(',').map(|s| s.trim()).for_each(|s| {
//         if let Some(cap) = FMT.captures(s) {
//             let quantity = cap[1].parse::<f64>().unwrap();
//             let price = cap[2].parse::<f64>().unwrap();
//             if &cap[3] == "B" { buy += quantity * price }
//             else { sell += quantity * price }
//         } else if !s.is_empty() {
//             bad.push(s)
//         }
//     });
//     if bad.is_empty() {
//         format!("Buy: {:.0} Sell: {:.0}", buy, sell)
//     } else {
//         format!("Buy: {:.0} Sell: {:.0}; Badly formed {}: {}",
//             buy, sell, bad.len(), bad.iter().map(|s| s.to_string() + " ;").collect::<String>())
//     }
// }
//
// fn balance_statement(l: &str) -> String {
//     let r = regex::Regex::new(r"^([^\s]+) (\d+) (\d+\.\d+) ([BS])$").unwrap();
//
//     l.split(',')
//         .map(str::trim)
//         .fold(([0.0; 2], vec![], "".to_owned()), |(mut t, mut e, _), o| {
//             match r.captures(o) {
//                 Some(c) => t[(&c[4] == "S") as usize] += c[2].parse::<f64>().unwrap() * c[3].parse::<f64>().unwrap(),
//                 None if !o.is_empty() => e.push(o.to_string()),
//                 _ => {}
//             }
//             (t, e.clone(), format!("Buy: {:.0} Sell: {:.0}{}", t[0], t[1],
//                 ["", &format!("; Badly formed {}: {} ;", e.len(), e.join(" ;"))]
//                 [(!e.is_empty()) as usize]))
//         }).2
// }
