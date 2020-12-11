use std::io::{stdin, BufRead};

fn main() {
    let numbers = numbers_from_stdin();
    let expected_sum = 2020;
    let (a, b) = find_two_numbers_with_sum(&numbers, expected_sum).unwrap();
    let first_answer = a * b;
    println!("first answer: {}", first_answer);
    let (c, d, e) = find_three_numbers_with_sum(&numbers, expected_sum).unwrap();
    let second_answer = c * d * e;
    println!("second answer: {}", second_answer);
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

fn find_three_numbers_with_sum(numbers: &Vec<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    match find_three_number_indices_with_sum(numbers, sum) {
        Some((i, j, k)) => Some((numbers[i], numbers[j], numbers[k])),
        _ => None
    }
}

fn find_three_number_indices_with_sum(numbers: &Vec<i64>, sum: i64) -> Option<(usize, usize, usize)> {
    for i in 0..numbers.len()-2 {
        for j in i+1..numbers.len() {
            for k in j+1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == sum {
                    return Some((i, j, k))
                }
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
