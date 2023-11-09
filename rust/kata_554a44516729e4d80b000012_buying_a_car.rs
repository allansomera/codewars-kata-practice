fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut calc = ((old + saving) as f64) - new as f64;
    println!("{:?}", calc);
    println!("perc {:?}", perc);
    let mut old_f = old as f64 * (1.0 - (perc / 100.0));
    println!("old_f {:?}", old_f);
    let mut new_f = new as f64 * (1.0 - (perc / 100.0));
    println!("new_f {:?}", new_f);
    let mut old_saving = old_f + saving as f64;
    let saved = 0.0;
    // while calc <= 0.0 {
    //     old_f = old_f as f64 * (1.0 - (perc / 100.0));
    //     println!("old_f {:?}", old_f);
    //     new_f = new_f as f64 * (1.0 - (perc / 100.0));
    //     println!("new_f {:?}", new_f);
    //     old_saving = old_f + saving as f64 + saved as f64;
    //
    //     calc = old_saving - new_f;
    //     println!("{:?}", calc);
    // }

    (0, 0)
}

fn main() {
    println!("{:?}", nb_months(2000, 8000, 1000, 1.5));
}
