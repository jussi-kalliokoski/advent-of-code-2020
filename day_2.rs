use std::io::{stdin, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let password_entries: Vec<PasswordEntry> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<PasswordEntry>().unwrap())
        .collect();
    let valid_passwords_count_by_sled_rental_rules = password_entries
        .iter()
        .filter(|password_entry| password_entry.valid_by_sled_rental_rules())
        .count();
    println!("first answer: {}", valid_passwords_count_by_sled_rental_rules);
    let valid_passwords_count = password_entries
        .iter()
        .filter(|password_entry| password_entry.valid())
        .count();
    println!("second answer: {}", valid_passwords_count);
}

struct PasswordEntry {
    usage_policy: UsagePolicy,
    password: String,
}

impl PasswordEntry {
    fn valid(self: &Self) -> bool {
        return self.usage_policy.allows(&self.password);
    }

    fn valid_by_sled_rental_rules(self: &Self) -> bool {
        return self.usage_policy.allows_by_sled_rental_rules(&self.password);
    }
}

#[derive(Debug)]
enum ParsePasswordEntryError {
    InvalidUsagePolicy(UsagePolicyError),
    InvalidColumnCount,
}

impl From<UsagePolicyError> for ParsePasswordEntryError {
    fn from(err: UsagePolicyError) -> Self {
        Self::InvalidUsagePolicy(err)
    }
}

impl FromStr for PasswordEntry {
    type Err = ParsePasswordEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = s.split(": ");
        let usage_policy = match columns.next() {
            Some(column) => column.parse::<UsagePolicy>(),
            _ => return Err(Self::Err::InvalidColumnCount)
        }?;
        let password = match columns.next() {
            Some(column) => column.to_string(),
            _ => return Err(Self::Err::InvalidColumnCount)
        };
        return Ok(Self{ usage_policy: usage_policy, password: password })
    }
}

struct UsagePolicy {
    character_allowed_uses: CharacterUsageRule,
    character: char,
}

impl UsagePolicy {
    fn allows(self: &Self, password: &String) -> bool {
        let left_char_matches = match password.chars().nth(self.character_allowed_uses.left - 1) {
            Some(c) => c == self.character,
            _ => false
        };
        let right_char_matches = match password.chars().nth(self.character_allowed_uses.right - 1) {
            Some(c) => c == self.character,
            _ => false
        };
        left_char_matches ^ right_char_matches
    }

    fn allows_by_sled_rental_rules(self: &Self, password: &String) -> bool {
        let character_count = password
            .chars()
            .filter(|c| self.character == *c)
            .count();
        self.character_allowed_uses.left <= character_count && self.character_allowed_uses.right >= character_count
    }
}

#[derive(Debug)]
enum UsagePolicyError {
    InvalidRange(ParseCharacterUsageRuleError),
    InvalidCharacter,
    InvalidColumnCount,
}

impl From<ParseCharacterUsageRuleError> for UsagePolicyError {
    fn from(err: ParseCharacterUsageRuleError) -> Self {
        Self::InvalidRange(err)
    }
}

impl FromStr for UsagePolicy {
    type Err = UsagePolicyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = s.split(' ');
        let character_allowed_uses = match columns.next() {
            Some(column) => column.parse::<CharacterUsageRule>(),
            _ => return Err(Self::Err::InvalidColumnCount)
        }?;
        let character_str = match columns.next() {
            Some(column) => column,
            _ => return Err(Self::Err::InvalidColumnCount)
        };
        if character_str.chars().count() != 1 {
            return Err(Self::Err::InvalidCharacter)
        }
        let character = character_str.chars().nth(0).unwrap();
        Ok(Self{ character_allowed_uses: character_allowed_uses, character: character })
    }
}

struct CharacterUsageRule {
    left: usize,
    right: usize,
}

#[derive(Debug)]
enum ParseCharacterUsageRuleError {
    InvalidInt(ParseIntError),
    InvalidRangeError,
}

impl From<ParseIntError> for ParseCharacterUsageRuleError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidInt(err)
    }
}

impl FromStr for CharacterUsageRule {
    type Err = ParseCharacterUsageRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split('-');
        let left = match numbers.next() {
            Some(number_str) => number_str.parse::<usize>(),
            _ => return Err(Self::Err::InvalidRangeError)
        }?;
        let right = match numbers.next() {
            Some(number_str) => number_str.parse::<usize>(),
            _ => return Err(Self::Err::InvalidRangeError)
        }?;
        match numbers.next() {
            Some(_) => return Err(Self::Err::InvalidRangeError),
            _ => {}
        }
        Ok(Self{ left: left, right: right })
    }
}
