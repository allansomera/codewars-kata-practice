fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut old_dep = (old as f64 / (1.0 + (perc / 100.0))) as f64;
    let mut new_dep = (new as f64 / (1.0 + (perc / 100.0))) as f64;
    let p = perc.clone();
    let mut calc: f64 = 0f64;
    let mut new_base = 0f64;
    calc = (old_dep - new_dep) - saving as f64;

    old_dep = (old_dep as f64 / (1.0 + (perc / 100.0))) as f64;
    new_dep = (new_dep as f64 / (1.0 + (perc / 100.0))) as f64;
    calc = (old_dep - new_dep) - saving as f64;
    new_base = calc.clone();
    println!("old_dep {:?}", old_dep);
    println!("new_dep {:?}", new_dep);
    println!("calc: {:?}", calc);
    println!("new_base {:?}", new_base);

    (0, 0)
}

fn main() {
    println!("{:?}", nb_months(2000, 8000, 1000, 1.5));
}
