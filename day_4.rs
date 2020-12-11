use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let mut passports_data = String::new();
    stdin().lock().read_to_string(&mut passports_data).unwrap();
    let passports = passports_data
        .split("\n\n")
        .map(|passport_data| passport_data.parse::<Passport>())
        .collect
        ::<Result<Vec<Passport>, ParsePassportError>>()
        .unwrap();
    let first_answer = passports
        .iter()
        .filter(|passport| passport.has_all_fields())
        .count();
    println!("first answer: {:?}", first_answer);
    let second_answer = passports
        .iter()
        .filter(|passport| passport.valid())
        .count();
    println!("second answer: {:?}", second_answer);
}

#[derive(Debug)]
struct Passport {
    fields: Vec<PassportField>,
}

impl Passport {
    fn valid(self: &Self) -> bool {
        self.has_all_fields() && self
            .fields
            .iter()
            .all(|field| field.valid())
    }

    fn has_all_fields(self: &Self) -> bool {
        let mut valid_fields: u8 = 0;
        for field in &self.fields {
            match field {
                PassportField::BirthYear(_) => valid_fields |= 1 << 0,
                PassportField::IssueYear(_) => valid_fields |= 1 << 1,
                PassportField::ExpirationYear(_) => valid_fields |= 1 << 2,
                PassportField::Height(_) => valid_fields |= 1 << 3,
                PassportField::HairColor(_) => valid_fields |= 1 << 4,
                PassportField::EyeColor(_) => valid_fields |= 1 << 5,
                PassportField::PassportID(_) => valid_fields |= 1 << 6,
                _ => {}
            }
        }
        valid_fields == 0b1111111
    }
}

#[derive(Debug)]
enum ParsePassportError {
    InvalidField(ParsePassportFieldError),
}

impl FromStr for Passport {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split_ascii_whitespace()
            .map(|column| column.parse::<PassportField>())
            .collect
            ::<Result<Vec<PassportField>, ParsePassportFieldError>>() {
                Ok(fields) => Ok(Self{ fields: fields }),
                Err(e) => Err(Self::Err::InvalidField(e))
            }
    }
}

#[derive(Debug)]
enum PassportField {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}

impl PassportField {
    fn valid(self: &Self) -> bool {
        match self {
            Self::BirthYear(data) => {
                match data.parse::<usize>() {
                    Ok(n) => n >= 1920 && n <= 2002,
                    _ => false
                }
            },
            Self::IssueYear(data) => {
                match data.parse::<usize>() {
                    Ok(n) => n >= 2010 && n <= 2020,
                    _ => false
                }
            },
            Self::ExpirationYear(data) => {
                match data.parse::<usize>() {
                    Ok(n) => n >= 2020 && n <= 2030,
                    _ => false
                }
            },
            Self::Height(data) => {
                if data.len() < 2 {
                    false
                } else {
                    match data[..data.len()-2].parse::<usize>() {
                        Ok(n) => {
                            match &data[data.len()-2..] {
                                "cm" => n >= 150 && n <= 193,
                                "in" => n >= 59 && n <= 76,
                                _ => false
                            }
                        },
                        _ => false
                    }
                }
            },
            Self::HairColor(data) => {
                if data.len() != 7 || &data[0..1] != "#" {
                    false
                } else {
                    data[1..]
                        .chars()
                        .all(|c| (c >= 'a' && c <= 'f') || (c >= '0' && c <= '9'))
                }
            },
            Self::EyeColor(data) => {
                match data.as_str() {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false
                }
            },
            Self::PassportID(data) => {
                if data.len() != 9 {
                    false
                } else {
                    data
                        .chars()
                        .all(|c| c >= '0' && c <= '9')
                }
            },
            Self::CountryID(_) => true,
        }
    }
}

#[derive(Debug)]
enum ParsePassportFieldError {
    MissingSeparator,
    UnknownField(String),
}

impl FromStr for PassportField {
    type Err = ParsePassportFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = s.split(':');
        let type_key = columns.next().unwrap();
        let data = match columns.next() {
            Some(v) => v.to_string(),
            _ => return Err(Self::Err::MissingSeparator)
        };
        match type_key {
            "byr" => Ok(Self::BirthYear(data)),
            "iyr" => Ok(Self::IssueYear(data)),
            "eyr" => Ok(Self::ExpirationYear(data)),
            "hgt" => Ok(Self::Height(data)),
            "hcl" => Ok(Self::HairColor(data)),
            "ecl" => Ok(Self::EyeColor(data)),
            "pid" => Ok(Self::PassportID(data)),
            "cid" => Ok(Self::CountryID(data)),
            _ => Err(Self::Err::UnknownField(type_key.to_string()))
        }
    }
}
