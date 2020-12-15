use std::collections::HashMap;

fn main() {
    assert!(count_sequence(vec![1,3,2], 2020) == 1);
    assert!(count_sequence(vec![2,1,3], 2020) == 10);
    assert!(count_sequence(vec![1,2,3], 2020) == 27);
    assert!(count_sequence(vec![2,3,1], 2020) == 78);
    assert!(count_sequence(vec![3,2,1], 2020) == 438);
    assert!(count_sequence(vec![3,1,2], 2020) == 1836);
    assert!(count_sequence(vec![0,3,6], 30000000) == 175594);
    assert!(count_sequence(vec![1,3,2], 30000000) == 2578);
    assert!(count_sequence(vec![2,1,3], 30000000) == 3544142);
    assert!(count_sequence(vec![1,2,3], 30000000) == 261214);
    assert!(count_sequence(vec![2,3,1], 30000000) == 6895259);
    assert!(count_sequence(vec![3,2,1], 30000000) == 18);
    assert!(count_sequence(vec![3,1,2], 30000000) == 362);
    println!("first answer: {:?}", count_sequence(vec![0,14,1,3,7,9], 2020));
    println!("second answer: {:?}", count_sequence(vec![0,14,1,3,7,9], 30000000));
}

fn count_sequence(seq: Vec<usize>, nth: usize) -> usize {
    let mut lookup: HashMap<usize, usize> = seq
        .iter()
        .enumerate()
        .take(seq.len() - 1)
        .map(|(i, v)| (*v, i))
        .collect();

    let mut prev = seq[seq.len()-1];
    for i in seq.len()..nth {
        let next = match lookup.get(&prev) {
            Some(j) => i - j - 1,
            _ => 0,
        };
        lookup.insert(prev, i - 1);
        prev = next;
    }
    prev
}
