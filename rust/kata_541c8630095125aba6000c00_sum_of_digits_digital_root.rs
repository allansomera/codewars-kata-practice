fn digital_root(n: i64) -> i64 {
    let mut total = digital_sum(n);

    while total.to_string().len() >= 2 {
        total = digital_sum(total)
    }
    total
}

fn digital_sum(n: i64) -> i64 {
    let v: Vec<i64> = n
        .to_string()
        .chars()
        .map(|x| x.to_string())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    // println!("{:?}", v);
    let a = v.into_iter().sum();
    a
}

fn main() {
    println!("{:?}", digital_root(16));
    println!("{:?}", digital_root(942));
    println!("{:?}", digital_root(132189));
    println!("{:?}", digital_root(493193));
}

// soln:
// fn digital_root(n: i64) -> i64 {
//     (n - 1) % 9 + 1
// }
//
// fn digital_root(n: i64) -> i64 {
//     if n/10==0 {n} else {digital_root(n%10 + n/10)}
// }
//
// fn digital_root(n: i64) -> i64 {
//     if n == 0 {
//         return 0;
//     }
//     match n % 9 {
//         0 => 9,
//         _ => n % 9,
//     }
// }
//
// fn digital_root(n: i64) -> i64 {
//     let mut n = n;
//     let mut vec:Vec<i64> = vec![];
//     while n.to_string().len() > 1 {
//         n.to_string().chars().map(|x| vec.push(x.to_string().parse::<i64>().unwrap())).for_each(drop);
//         n = vec.iter().sum();
//         vec = vec![];
//     }
//     n
// }
//
// fn digital_root(n: i64) -> i64 {
//     if n < 10 {
//         return n;
//     }
//     let sum: i64 = n
//         .to_string()
//         .chars()
//         .map(|x| x.to_string().parse::<i64>().unwrap())
//         .sum();
//     digital_root(sum)
// }
//
// fn digital_root(n: i64) -> i64 {
//     let sum = n
//         .to_string()
//         .chars()
//         .map(|c| c.to_digit(10u32).unwrap() as i64)
//         .sum();
//
//     if sum > 9 {
//         return digital_root(sum);
//     }
//
//     sum
// }
//
// fn digital_root(n: i64) -> i64 {
// 	let result = format!("{}",n)
// 		.chars()
// 		.map(|c|{c.to_string().parse().unwrap()})
// 		.reduce(|sum, x|{ sum + x })
// 		.unwrap();
// 	if result > 9 { digital_root(result)} else{result}
// }
//
// fn digital_root(n: i64) -> i64 {
//     let mut result: i64 = 0;
//
//     let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
//     for d in digits {
//         result = result + d;
//     }
//     match result {
//         _ if result >= 10 => digital_root(result),
//         a => a
//     }
// }
//
// fn digital_root(mut n: i64) -> i64 {
//     while n.to_string().len() > 1 {
//         n = n.to_string().chars().map(|x| x.to_string().parse::<i64>().unwrap()).sum::<i64>();
//     }
//     n
// }
//
// fn digital_root(n: i64) -> i64 {
//     let sum = n.to_string()
//         .chars()
//         .into_iter()
//         .map(|x| x.to_digit(10).unwrap_or(0) as i64)
//         .sum::<i64>();
//     match sum.to_string().len() == 1 {
//         true => sum,
//         false => digital_root(sum)
//     }
// }
