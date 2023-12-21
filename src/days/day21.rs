use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    line: usize,
    col: usize,
}

impl Coordinate {
    fn neighbors(&self, map: &Vec<Vec<Tile>>) -> Vec<Coordinate> {
        let (height, width) = (map.len(), map[0].len());
        let mut neighbors = vec![];
        if self.line > 0 {
            let mut coord = self.clone();
            coord.line -= 1;
            if map[coord.line][coord.col] == Tile::Garden {
                neighbors.push(coord);
            }
        }
        if self.line < height - 1 {
            let mut coord = self.clone();
            coord.line += 1;
            if map[coord.line][coord.col] == Tile::Garden {
                neighbors.push(coord);
            }
        }
        if self.col > 0 {
            let mut coord = self.clone();
            coord.col -= 1;
            if map[coord.line][coord.col] == Tile::Garden {
                neighbors.push(coord);
            }
        }
        if self.col < width - 1 {
            let mut coord = self.clone();
            coord.col += 1;
            if map[coord.line][coord.col] == Tile::Garden {
                neighbors.push(coord);
            }
        }
        neighbors
    }
}

fn get_input() -> (Coordinate, Vec<Vec<Tile>>) {
    let input = fs::read_to_string(Path::new("./input/day21.input"))
        .expect("Something went wrong with the input");
    let mut start = Coordinate { line: 0, col: 0 };
    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(|(col_no, elem)| match elem {
                    '.' => Tile::Garden,
                    '#' => Tile::Rock,
                    'S' => {
                        start.line = line_no;
                        start.col = col_no;
                        Tile::Garden
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (start, map)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (start, map) = get_input();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut visitable_gardens = 0;
    visited.insert(start.clone());
    let mut possible_coords = vec![start];
    for step in 0..=64 {
        if step % 2 == 0 {
            visitable_gardens += possible_coords.len();
        }
        let mut next_coords = vec![];
        for coord in possible_coords {
            let candidates = coord.neighbors(&map);
            for candidate in candidates {
                if visited.insert(candidate.clone()) {
                    next_coords.push(candidate);
                }
            }
        }
        possible_coords = next_coords;
    }
    println!(
        "After taking 64 steps, the elf can reach up to {} gardens",
        visitable_gardens
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
