fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let old_dep = (old as f64 / (1.0 + (perc / 100.0))) as f64;
    let new_dep = (new as f64 / (1.0 + (perc / 100.0))) as f64;

    // println!("old_dep = {:?} / ({:?} + {:?})", old, 1.0, perc / 100.0);
    println!("old_dep {:?}", old_dep);
    println!("new_dep {:?}", new_dep);
    (0, 0)
}

fn main() {
    println!("{:?}", nb_months(2000, 8000, 1000, 1.5));
}
