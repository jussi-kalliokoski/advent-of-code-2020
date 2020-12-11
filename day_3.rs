use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    let mut map_data = String::new();
    stdin().lock().read_to_string(&mut map_data).unwrap();
    let map = map_data.parse::<Map>().unwrap();
    let first_answer = map.count_tiles_for_slope(Tile::Tree, 3, 1);
    println!("{}", first_answer);
    let second_answer =
        map.count_tiles_for_slope(Tile::Tree, 1, 1)
        * first_answer
        * map.count_tiles_for_slope(Tile::Tree, 5, 1)
        * map.count_tiles_for_slope(Tile::Tree, 7, 1)
        * map.count_tiles_for_slope(Tile::Tree, 1, 2);
    println!("{}", second_answer);
}

#[derive(Debug)]
struct Map {
    rows: Vec<MapRow>,
}

impl Map {
    fn count_tiles_for_slope(self: &Self, tile: Tile, slope_w: usize, slope_h: usize) -> usize {
        let mut tiles_encountered = 0;
        let mut x = 0;
        let mut y = 0;
        loop {
            match self.xy(x, y) {
                Some(t) => {
                    if *t == tile {
                        tiles_encountered += 1;
                    }
                },
                _ => return tiles_encountered
            }
            x += slope_w;
            y += slope_h;
        }
    }

    fn xy(self: &Self, x: usize, y: usize) -> Option<&Tile> {
        let row = match self.rows.get(y) {
            Some(row) => row,
            _ => return None
        };
        row.tiles.get(x % row.tiles.len())
    }
}

#[derive(Debug)]
enum ParseMapError {
    InvalidRow(ParseMapRowError),
}

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .lines()
            .map(|line| line.parse::<MapRow>())
            .collect
            ::<Result<Vec<MapRow>, ParseMapRowError>>() {
                Ok(rows) => Ok(Self{ rows: rows }),
                Err(e) => Err(Self::Err::InvalidRow(e)),
            }
    }
}

#[derive(Debug)]
struct MapRow {
    tiles: Vec<Tile>,
}

#[derive(Debug)]
enum ParseMapRowError {
    InvalidTile(ParseTileError),
}

impl FromStr for MapRow {
    type Err = ParseMapRowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .chars()
            .map(|c| c.to_string().parse::<Tile>())
            .collect
            ::<Result<Vec<Tile>, ParseTileError>>() {
                Ok(tiles) => Ok(Self{ tiles: tiles }),
                Err(e) => Err(Self::Err::InvalidTile(e)),
            }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Tree,
    Empty,
}

#[derive(Debug)]
enum ParseTileError {
    UnrecognizedTile(String),
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Tile::Tree),
            "." => Ok(Tile::Empty),
            _ => Err(Self::Err::UnrecognizedTile(s.to_string())),
        }
    }
}
