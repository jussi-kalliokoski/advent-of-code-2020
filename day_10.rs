use std::io::{stdin, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut adapters = input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect
        ::<Vec<usize>>();
    adapters.sort();
    let built_in_adapter = adapters.last().unwrap() + 3;
    adapters.push(built_in_adapter);
    let first_result = adapters.iter().fold([0 as usize; 4], |mut acc, adapter| {
        let diff = *adapter - acc[0];
        acc[0] = *adapter;
        acc[diff] += 1;
        acc
    });
    let first_answer = first_result[1] * first_result[3];
    println!("first answer: {:?}", first_answer);
    let second_answer = count_distinct_valid_combinations(&adapters, 0, 0, &mut HashMap::new());
    println!("second answer: {:?}", second_answer);
}

fn count_distinct_valid_combinations(adapters: &Vec<usize>, offset: usize, first: usize, mut memo: &mut HashMap<(usize, usize), usize>) -> usize {
    match memo.get(&(offset, first)) {
        Some(result) => return *result,
        _ => {}
    }
    let mut prev = first;
    let mut count = 1;
    for i in offset..adapters.len()-2 {
        let next = adapters[i+1];
        if next - prev <= 3 {
            count += count_distinct_valid_combinations(&adapters, i + 1, prev, &mut memo);
        }
        prev = adapters[i];
    }
    memo.insert((offset, first), count);
    count
}
