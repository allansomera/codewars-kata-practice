fn derive(coefficient: u32, exponent: u32) -> String {
    let mut vec_s: Vec<u32> = Vec::new();
    vec_s.push(coefficient * exponent);
    vec_s.push(exponent - 1);
    vec_s
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("x^")
}

fn main() {
    println!("{:?}", derive(7, 8));
}

// soln:
// fn derive(coefficient: u32, exponent: u32) -> String {
//     format!("{}x^{}", coefficient * exponent, exponent - 1)
// }
//
// fn derive(coefficient: u32, exponent: u32) -> String {
//    [(coefficient* exponent).to_string(),"x^".to_string(),(exponent-1).to_string()].to_vec().concat()
// }
//
// fn derive(coefficient: u32, exponent: u32) -> String {
//     let coef: u32 = coefficient * exponent;
//     let expo: u32 = exponent-1;
//     return format!("{}x^{}",coef,expo);
// }
//
// fn derive(coefficient: u32, exponent: u32) -> String {
//     format!("{}x^{}", coefficient * exponent, exponent -1).to_string()
// }
//
// fn derive(coefficient: u32, exponent: u32) -> String {
//     format!("{coeff}x^{exp}", coeff=coefficient*exponent, exp=exponent-1)
// }
