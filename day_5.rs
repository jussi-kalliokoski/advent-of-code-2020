use std::io::{stdin, BufRead};
use std::str::FromStr;

fn main() {
    let mut taken_seats = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<SeatID>())
        .collect
        ::<Result<Vec<SeatID>, ParseSeatIDError>>()
        .unwrap();
    taken_seats.sort();
    let first_answer = taken_seats.iter().last().unwrap();
    println!("first answer: {:?}", first_answer);
    let second_answer = find_free_seat(taken_seats).unwrap();
    println!("second answer: {:?}", second_answer);
}

fn find_free_seat(taken_seats: Vec<SeatID>) -> Option<SeatID> {
    let mut prev = taken_seats[0].0;
    for seat in taken_seats[1..].iter() {
        if seat.0 > prev + 1 {
            return Some(SeatID(prev + 1));
        }
        prev = seat.0;
    }
    None
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct SeatID(usize);

#[derive(Debug)]
enum ParseSeatIDError {
    InvalidChar(char),
    InvalidLength,
}

impl FromStr for SeatID {
    type Err = ParseSeatIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            println!("{}", s);
            return Err(Self::Err::InvalidLength)
        }
        let id = s[0..7].chars().enumerate().try_fold(0 as usize, |acc, (i, c)| {
            match c {
                'B' => Ok(acc | (1 << (9-i))),
                'F' => Ok(acc),
                _ => Err(Self::Err::InvalidChar(c))
            }
        })? + s[7..].chars().enumerate().try_fold(0 as usize, |acc, (i, c)| {
            match c {
                'R' => Ok(acc | (1 << (2-i))),
                'L' => Ok(acc),
                _ => Err(Self::Err::InvalidChar(c))
            }
        })?;
        Ok(Self(id))
    }
}
