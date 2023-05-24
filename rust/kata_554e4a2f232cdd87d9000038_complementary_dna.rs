fn dna_strand(dna: &str) -> String {
    let s: String = dna
        .chars()
        .map(|x| match x {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => x,
        })
        .collect::<String>();
    s
}

fn main() {
    println!("{:?}", dna_strand("ATTGC"));
}

// soln:
// fn dna_strand(dna: &str) -> String {
//   dna.chars()
//     .filter_map(|c| match c {
//         'A' => Some('T'),
//         'T' => Some('A'),
//         'G' => Some('C'),
//         'C' => Some('G'),
//         _ => None,
//         })
//     .collect::<String>()
// }
//
// fn dna_strand(dna: &str) -> String {
//     let mut result = String::with_capacity(dna.len());
//     for c in dna.chars() {
//         match c {
//             'A' => result.push('T'),
//             'T' => result.push('A'),
//             'C' => result.push('G'),
//             'G' => result.push('C'),
//             _ => panic!(),
//         }
//     }
//
//     result
// }
//
// fn dna_strand(dna: &str) -> String {
//   dna.chars().map(|i| match i {
//       'A' => 'T',
//       'T' => 'A',
//       'C' => 'G',
//       'G' => 'C',
//       _ => panic!()
//   }).collect::<String>()
// }
