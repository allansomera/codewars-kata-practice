fn dna_to_rna(dna: &str) -> String {
    let d: String = dna
        .chars()
        .map(|x| match x {
            'T' => 'U',
            _ => x,
        })
        .collect();
    d
}

fn main() {
    println!("{:?}", dna_to_rna("GCAT"));
}

// soln:
// fn dna_to_rna(dna: &str) -> String {
//  dna.replace("T", "U")
// }
//
// fn dna_to_rna(dna: &str) -> String {
//     let mut res = String::new();
//     for s in dna.chars() {
//         match s {
//             'T' => res.push('U'),
//             _ => res.push(s),
//         }
//     }
//     res
// }
//
// fn dna_to_rna(dna: &str) -> String {
//    str::replace(dna, "T", "U").to_string()
// }
