use std::io::{stdin, BufRead};

fn main() {
    let numbers = numbers_from_stdin();
    let preamble_len = 25;
    let first_non_summable = find_first_with_no_combination_sum(&numbers, preamble_len).unwrap();
    println!("first answer: {}", first_non_summable);
    let (first, last) = find_range_with_sum(&numbers, first_non_summable).unwrap();
    let smallest = numbers[first..=last].iter().min().unwrap();
    let largest = numbers[first..=last].iter().max().unwrap();
    let encryption_weakness = smallest + largest;
    println!("second answer: {}", encryption_weakness);
}

fn find_first_with_no_combination_sum(numbers: &Vec<i64>, preamble_len: usize) -> Option<i64> {
    for i in preamble_len..numbers.len() {
        let number = numbers[i];
        if !contains_two_numbers_with_sum(numbers, number) {
            return Some(number)
        }
    }
    None
}

fn contains_two_numbers_with_sum(numbers: &Vec<i64>, sum: i64) -> bool {
    match find_two_numbers_with_sum(numbers, sum) {
        Some((_, _)) => true,
        _ => false
    }
}

fn find_two_numbers_with_sum(numbers: &Vec<i64>, sum: i64) -> Option<(i64, i64)> {
    match find_two_number_indices_with_sum(numbers, sum) {
        Some((i, j)) => Some((numbers[i], numbers[j])),
        _ => None
    }
}

fn find_two_number_indices_with_sum(numbers: &Vec<i64>, sum: i64) -> Option<(usize, usize)> {
    for i in 0..numbers.len()-1 {
        for j in i+1..numbers.len() {
            if numbers[i] + numbers[j] == sum {
                return Some((i, j))
            }
        }
    }
    None
}

fn find_range_with_sum(numbers: &Vec<i64>, sum: i64) -> Option<(usize, usize)> {
    for first in 0..numbers.len()-1 {
        for last in first+1..numbers.len() {
            if numbers[first..=last].iter().sum::<i64>() == sum {
                return Some((first, last))
            }
        }
    }
    None
}

fn numbers_from_stdin() -> Vec<i64> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}
