fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }
    return false;
}

fn main() {
    println!("{:?}", is_leap_year(1234));
    println!("{:?}", is_leap_year(1984));
    println!("{:?}", is_leap_year(2000));
    println!("{:?}", is_leap_year(2010));
    println!("{:?}", is_leap_year(2013));
}

// soln:
// fn is_leap_year(year: i32) -> bool {
//     match year {
//         x if x % 400 == 0 => true,
//         x if x % 100 == 0 => false,
//         x if x % 4 == 0 => true,
//         _ => false,
//     }
// }
//
// fn is_leap_year(year: i32) -> bool {
//     year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
// }
//
// fn is_leap_year(year: i32) -> bool {
//     if year % 400 == 0 {
//         true
//     } else if year % 100 == 0 {
//         false
//     } else {
//         year % 4 == 0
//     }
// }
