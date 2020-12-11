use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let seat_map = input.parse::<SeatMap>().unwrap();
    let first_answer = seat_map
        .transform(|seat_map, x, y, seat_state| {
            match seat_state {
                SeatState::Empty => {
                    if seat_map.count_adjacent_seats_of_state(x, y, SeatState::Taken) == 0 {
                        return SeatState::Taken
                    }
                },
                SeatState::Taken => {
                    if seat_map.count_adjacent_seats_of_state(x, y, SeatState::Taken) >= 4 {
                        return SeatState::Empty
                    }
                },
                _ => {}
            }
            seat_state
        })
        .count_seats_of_state(SeatState::Taken);
    println!("first answer: {}", first_answer);
    let second_answer = seat_map
        .transform(|seat_map, x, y, seat_state| {
            match seat_state {
                SeatState::Empty => {
                    if seat_map.count_visible_seats_of_state(x, y, SeatState::Taken) == 0 {
                        return SeatState::Taken
                    }
                },
                SeatState::Taken => {
                    if seat_map.count_visible_seats_of_state(x, y, SeatState::Taken) >= 5 {
                        return SeatState::Empty
                    }
                },
                _ => {}
            }
            seat_state
        })
        .count_seats_of_state(SeatState::Taken);
    println!("second answer: {}", second_answer);
}

#[derive(Debug, Clone)]
struct SeatMap {
    row_width: usize,
    states: Vec<SeatState>,
}

impl SeatMap {
    #[inline(always)]
    fn get(self: &Self, x: usize, y: usize) -> SeatState {
        self.states[y * self.row_width + x]
    }

    #[inline(always)]
    fn get_with_delta(self: &Self, x: usize, y: usize, xd: isize, yd: isize) -> Option<SeatState> {
        let ix = x as isize + xd;
        let iy = y as isize + yd;
        if ix < 0 || iy < 0 {
            return None
        }
        let ux = ix as usize;
        let uy = iy as usize;
        if ux >= self.row_width || uy >= self.height() {
            return None
        }
        Some(self.get(ux, uy))
    }

    #[inline(always)]
    fn set(self: &mut Self, x: usize, y: usize, state: SeatState) {
        self.states[y * self.row_width + x] = state;
    }

    #[inline(always)]
    fn height(self: &Self) -> usize {
        self.states.len() / self.row_width
    }

    fn count_seats_of_state(self: &Self, state: SeatState) -> usize {
        self.states.iter().filter(|s| **s == state).count()
    }

    fn count_visible_seats_of_state(self: &Self, x: usize, y: usize, state: SeatState) -> usize {
        let mut count = 0;
        for yd in -1..=1 {
            for xd in -1..=1 {
                if xd != 0 || yd != 0 {
                    let mut i = 1;
                    'find_non_floor: loop {
                        match self.get_with_delta(x, y, xd * i, yd * i) {
                            Some(s) => {
                                if s == state {
                                    count += 1;
                                } else if s == SeatState::Floor {
                                    i += 1;
                                    continue 'find_non_floor;
                                }
                            },
                            _ => {}
                        }
                        break;
                    }
                }
            }
        }
        count
    }

    fn count_adjacent_seats_of_state(self: &Self, x: usize, y: usize, state: SeatState) -> usize {
        let mut count = 0;
        for yd in -1..=1 {
            for xd in -1..=1 {
                if xd != 0 || yd != 0 {
                    match self.get_with_delta(x, y, xd, yd) {
                        Some(s) => {
                            if s == state {
                                count += 1;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn print(self: &Self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}\n", self.states.chunks(self.row_width).map(|s| {
            s.iter().map(|s| match s {
                SeatState::Floor => ".",
                SeatState::Empty => "L",
                SeatState::Taken => "#",
            }.to_string()).collect::<Vec<String>>().join("")
        }).collect::<Vec<String>>().join("\n"));
    }

    fn transform(self: &Self, rearrange: fn(&Self, usize, usize, SeatState) -> SeatState) -> Self {
        let mut scratch = [self.clone(), self.clone()];
        let mut dst = 0;
        loop {
            let src = dst;
            dst ^= 1;
            let mut changes = 0;
            for y in 0..self.height() {
                for x in 0..self.row_width {
                    let old = scratch[src].get(x, y);
                    let new = rearrange(&scratch[src], x, y, old);
                    scratch[dst].set(x, y, new);
                    if old != new {
                        changes += 1;
                    }
                }
            }
            if changes == 0 {
                return Self{ row_width: self.row_width, states: scratch[dst].states.to_owned() }
            }
        }
    }
}

#[derive(Debug)]
enum ParseSeatMapError {
    InvalidSeatState(ParseSeatStateError),
}

impl FromStr for SeatMap {
    type Err = ParseSeatMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_width = 0;
        let states = match s
            .lines()
            .flat_map(|line| {
                row_width = line.len();
                line
                    .chars()
                    .map(|c| c.to_string().parse::<SeatState>())
            })
            .collect
            ::<Result<Vec<SeatState>, ParseSeatStateError>>() {
                Ok(s) => s,
                Err(e) => return Err(Self::Err::InvalidSeatState(e))
            };
        Ok(Self{ row_width: row_width, states: states })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Taken,
}

#[derive(Debug)]
enum ParseSeatStateError {
    InvalidState(String),
}

impl FromStr for SeatState {
    type Err = ParseSeatStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Floor),
            "L" => Ok(Self::Empty),
            "#" => Ok(Self::Taken),
            _ => Err(Self::Err::InvalidState(s.to_string()))
        }
    }
}
