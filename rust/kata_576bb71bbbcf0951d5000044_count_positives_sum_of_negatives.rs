fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() == 0 {
        return vec![];
    }
    let vv: Vec<i32> = vec![
        input
            .iter()
            .filter(|x: &&i32| x.is_positive())
            .map(|x: &i32| *x)
            .collect::<Vec<i32>>()
            .len() as i32,
        input
            .iter()
            .filter(|x: &&i32| x.is_negative())
            .map(|x: &i32| *x)
            .sum(),
    ];
    vv
}

fn main() {
    println!(
        "{:?}",
        count_positives_sum_negatives(
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15]).to_vec()
        )
    );
}

soln:
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
  if input.is_empty() {
  	return vec![];
  }

	input.iter().fold(vec![0, 0], |mut acc, &x| {
    	if x > 0 {
        acc[0] += 1;
      } else {
        acc[1] += x;
      }
      acc
	})
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }
    
    vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum()
    ]
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
  if input.is_empty() { return vec![]; }
  let count_positives = input.iter().filter(|&&x| x.is_positive()).count() as i32;
  let sum_negatives = input.iter().filter(|&&x| x.is_negative()).sum();
  vec![count_positives, sum_negatives]
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() { vec![] }
    else { vec![
        input.iter().filter(|i| i.is_positive()).count() as i32, 
        input.iter().filter(|i| i.is_negative()).sum()
    ]}
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() { return Vec::new() }
    let mut v = vec![0, 0];
    v[0] = input
        .iter()
        .filter(|e| e.is_positive())
        .map(|e| e.to_owned())
        .count() as i32;
    v[1] = input
        .iter()
        .filter(|e| e.is_negative())
        .map(|e| e.to_owned())
        .sum();
    v
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {return vec![]};
    let (p,n) : (Vec<i32>, Vec<i32>) = input.iter().partition(|&x| x.is_positive());
    vec![p.iter().count() as i32, n.iter().sum()]
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() { return vec![] }
    input.iter().fold(vec![0, 0], |a, &x| if x <= 0 {vec![a[0], a[1]+x]} else {vec![a[0]+1, a[1]]})
}

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() { return vec![]; }

    let positive: Vec<i32> = input.iter().filter( |i| i.is_positive()).cloned().collect();
    let negative: Vec<i32> = input.iter().filter( |i| i.is_negative()).cloned().collect();
    let sum: i32 = negative.iter().sum();

    vec![positive.len() as i32, sum]
}
