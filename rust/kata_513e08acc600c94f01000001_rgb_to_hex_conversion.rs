fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut rgb_vec = Vec::new();
    rgb_vec.push(r);
    rgb_vec.push(g);
    rgb_vec.push(b);

    let converted: Vec<String> = rgb_vec
        .into_iter()
        .map(|x| match x {
            n if n < 0 => format!("{:02X}", 0),
            n if n > 255 => format!("{:02X}", 255),
            _ => format!("{:X}", x),
        })
        .collect::<Vec<String>>();

    println!("{:?}", converted.join(""));

    // vec![r, g, b]
    //     .iter()
    //     .map(|x| format!("{:02X}", x))
    //     .collect::<Vec<String>>()
    //     .join("")

    // vec![r, g, b]
    //     .into_iter()
    //     .map(|x| match x {
    //         n if n < 0 => format!("{:02X}", 0),
    //         n if n > 255 => format!("{:02X}", 255),
    //         _ => format!("{:02X}", x)
    //     })
    //     .collect::<Vec<String>>()
    //     .join("")

    "to_string()".to_string()
}

fn main() {
    println!("{:?}", rgb(0, 0, 0));
    println!("{:?}", rgb(255, 255, 255));
    println!("{:?}", rgb(-20, 275, 125));
}

// soln:
//
// fn rgb(r: i32, g: i32, b: i32) -> String {
//     format!(
//         "{:02X}{:02X}{:02X}",
//         r.clamp(0, 255),
//         g.clamp(0, 255),
//         b.clamp(0, 255)
//     )
// }
//
// fn rgb(r: i32, g: i32, b: i32) -> String {
//   format!("{0:02X}{1:02X}{2:02X}", r.min(255).max(0), g.min(255).max(0), b.min(255).max(0))
// }
//
// fn rgb(r: i32, g: i32, b: i32) -> String {
//     let r = num::clamp(r, 0, 255);
//     let g = num::clamp(g, 0, 255);
//     let b = num::clamp(b, 0, 255);
//     format!("{:06X}", (r << 16) + (g << 8) + b)
// }
//
// fn rgb(r: i32, g: i32, b: i32) -> String {
//     [r, g, b].iter()
//         .map(|&x| if x < 0        {0}
//                   else if x > 255 {255}
//                   else            {x})
//         .map(|x| format!("{:02X}", x))
//         .collect()
// }
//
// fn rgb(r: i32, g: i32, b: i32) -> String {
//     [r,g,b].iter()
//         .map(|&n|
//             format!("{:02X}", n.clamp(0,255))
//     ).collect()
//
// }
//
// fn rgb(r: i32, g: i32, b: i32) -> String {
//   [r, g, b].map(|i| format!("{:0>2X}", i.max(0).min(255))).join("")
// }
//
