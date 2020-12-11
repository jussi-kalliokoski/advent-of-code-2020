use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let first_answer: u32 = input
        .split("\n\n")
        .map(|group_data| {
            group_data
                .lines()
                .map(|answers_data| {
                    answers_data
                        .chars()
                        .map(|c| 1 << (c as u32 - 'a' as u32))
                        .fold(0 as u32, |acc, v| acc | v)
                })
                .fold(0 as u32, |acc, v| acc | v)
                .count_ones()
        })
        .sum();
    println!("first answer: {}", first_answer);
    let second_answer: u32 = input
        .split("\n\n")
        .map(|group_data| {
            group_data
                .lines()
                .map(|answers_data| {
                    answers_data
                        .chars()
                        .map(|c| 1 << (c as u32 - 'a' as u32))
                        .fold(0 as u32, |acc, v| acc | v)
                })
                .fold(!(0 as u32), |acc, v| acc & v)
                .count_ones()
        })
        .sum();
    println!("second answer: {}", second_answer);
}
