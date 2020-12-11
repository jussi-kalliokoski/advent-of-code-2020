use std::io::{stdin, BufRead};
use std::collections::HashMap;

fn main() {
    let rules: Vec<Rule> = stdin()
        .lock()
        .lines()
        .map(|line| {
            let preprocessed = line
                .unwrap()
                .to_string()
                .trim_end_matches(".")
                .replace("bags", "bag");
            let columns: Vec<&str> = preprocessed.split(" bag contain ").collect();
            let contents = match columns[1] {
                "no other bag" => vec![],
                raw => raw
                    .split(", ")
                    .map(|str| {
                        let normalized = str.replace(" bag", "");
                        let words: Vec<&str> = normalized.split(" ").collect();
                        let count = words[0].parse::<usize>().unwrap();
                        let attribute = words[1..].join(" ");
                        BagCount{ attribute: attribute, count: count }
                    })
                    .collect()
            };
            Rule{
                target: columns[0].replace(" bag", ""),
                contents: contents,
            }
        })
        .collect();
    let mut lookup = HashMap::new();
    for i in 0..rules.len() {
        lookup.insert(rules[i].target.clone(), i);
    }
    let mut first_answer = 0;
    let attribute_to_find = "shiny gold".to_string();
    for i in 0..rules.len() {
        if rules[i].allows(&attribute_to_find, &lookup, &rules) {
            first_answer += 1;
        }
    }
    println!("{}", first_answer);
    let second_answer = rules[*lookup.get(&attribute_to_find).unwrap()].count_contained_bags(&lookup, &rules);
    println!("{}", second_answer);
}

#[derive(Debug)]
struct Rule {
    target: String,
    contents: Vec<BagCount>,
}

impl Rule {
    fn allows(self: &Self, attribute: &String, lookup: &HashMap<String, usize>, rules: &Vec<Rule>) -> bool {
        for i in 0..self.contents.len() {
            if self.contents[i].attribute == *attribute {
                return true
            }
            match lookup.get(&self.contents[i].attribute) {
                Some(k) => {
                    if rules[*k].allows(attribute, lookup, rules) {
                        return true
                    }
                },
                _ => {}
            }
        }
        return false
    }

    fn count_contained_bags(self: &Self, lookup: &HashMap<String, usize>, rules: &Vec<Rule>) -> usize {
        let mut count = 0;
        for i in 0..self.contents.len() {
            let contained_count = match lookup.get(&self.contents[i].attribute) {
                Some(k) => rules[*k].count_contained_bags(&lookup, &rules),
                _ => 0
            };
            count += contained_count * self.contents[i].count + self.contents[i].count
        }
        count
    }
}

#[derive(Debug)]
struct BagCount {
    attribute: String,
    count: usize,
}
