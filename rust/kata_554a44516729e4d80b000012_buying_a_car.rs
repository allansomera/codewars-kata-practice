fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    // let mut calc = ((old + saving) as f64) - new as f64;
    // println!("{:?}", calc);
    // // println!("perc {:?}", perc);
    // let mut old_f = old as f64 * (1.0 - (perc / 100.0));
    // println!("old_f {:?}", old_f);
    // let mut new_f = new as f64 * (1.0 - (perc / 100.0));
    // println!("new_f {:?}", new_f);
    // let mut perc_new = perc.clone();
    // println!("perc_new: {:?}", perc_new);
    // let mut old_saving = old_f + saving as f64;
    // let mut saved = 0.0;
    // // println!("{:?}", old_saving - new_f);
    // saved += saving as f64;
    // let mut target: bool = true;
    // let mut month = 1;
    // let mut count = 1;
    // // let oo = old_f as f64 * (1.0 - (perc / 100.0));
    // // let nf = new_f as f64 * (1.0 - (perc / 100.0));
    // // let oo_s = oo + saved + saving as f64 - nf;
    // // println!("{:?}", oo_s);
    //
    // while target {
    //     println!("count: {:?}", count);
    //     old_f = old_f as f64 * (1.0 - (perc_new / 100.0));
    //     // println!("old_f: {:?}", old_f);
    //     new_f = new_f as f64 * (1.0 - (perc_new / 100.0));
    //     println!("new_f: {:?}", new_f);
    //     old_saving = old_f + saving as f64 + saved as f64;
    //     println!("old_saving: {:?}", old_saving);
    //     println!("perc_new: {:?}", perc_new);
    //     calc = old_saving - new_f;
    //     println!("calc: {:?}", calc);
    //     println!("month: {:?}", month);
    //     month += 1;
    //     saved += saving as f64;
    //     count += 1;
    //     if month % 2 == 0 {
    //         perc_new += 0.5;
    //     }
    //     if calc > 0.0 {
    //         target = false;
    //     }
    // }
    let mut calc = ((old + saving) as f64) - new as f64;
    println!("calc: {:?}", calc);
    let mut old_first = old as f64 * (1.0 - (perc / 100.0));
    println!("first: {:?}", old_first);
    let mut new_f = new as f64 * (1.0 - (perc / 100.0));
    println!("new_f {:?}", new_f);
    let mut old_saving = old_first + saving as f64;
    println!("old_saving {:?}", old_saving);
    let mut saved = 0.0;
    println!("saved {:?}", saved);
    saved += saving as f64;
    println!("saved {:?}", saved);
    (0, 0)
}

fn main() {
    println!("{:?}", nb_months(2000, 8000, 1000, 1.5));
}
