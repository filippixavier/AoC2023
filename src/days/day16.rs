use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

#[derive(Debug)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

impl Beam {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        Beam {
            position,
            direction,
        }
    }
    fn rotate_beam(&mut self, tile: char) -> Option<Self> {
        let position = self.position;
        match tile {
            '.' => None,
            '/' => {
                self.direction = match self.direction {
                    Up => Right,
                    Right => Up,
                    Down => Left,
                    Left => Down,
                };
                None
            }
            '-' => {
                if self.direction == Up || self.direction == Down {
                    self.direction = Left;
                    Some(Self::new(position, Right))
                } else {
                    None
                }
            }
            '\\' => {
                self.direction = match self.direction {
                    Up => Left,
                    Right => Down,
                    Down => Right,
                    Left => Up,
                };
                None
            }
            '|' => {
                if self.direction == Left || self.direction == Right {
                    self.direction = Up;
                    Some(Self::new(position, Down))
                } else {
                    None
                }
            }
            _ => unreachable!(),
        }
    }

    fn move_beam(mut self, width: usize, height: usize) -> Option<Self> {
        let mut position = (self.position.0 as isize, self.position.1 as isize);
        match self.direction {
            Up => {
                position.0 -= 1;
            }
            Down => {
                position.0 += 1;
            }
            Left => {
                position.1 -= 1;
            }
            Right => {
                position.1 += 1;
            }
        }
        if position.0 >= 0
            && position.0 < height as isize
            && position.1 >= 0
            && position.1 < width as isize
        {
            self.position = (position.0 as usize, position.1 as usize);
            Some(self)
        } else {
            None
        }
    }

    fn tile_effect(mut self, map: &Vec<Vec<char>>) -> (Option<Self>, Option<Self>) {
        let (height, width) = (map.len(), map[0].len());
        let tile = map[self.position.0][self.position.1];
        let other_beam = self.rotate_beam(tile);

        (
            self.move_beam(width, height),
            if let Some(beam) = other_beam {
                beam.move_beam(width, height)
            } else {
                None
            },
        )
    }
}

fn get_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(Path::new("./input/day16.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input();
    let mut cycle_detector: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut beams = vec![Beam::new((0, 0), Right)];

    while let Some(beam) = beams.pop() {
        if !cycle_detector.insert((beam.position, beam.direction)) {
            continue;
        }
        visited.insert(beam.position);
        let moved_beams = beam.tile_effect(&map);
        if let Some(beam) = moved_beams.0 {
            beams.push(beam);
        }
        if let Some(beam) = moved_beams.1 {
            beams.push(beam);
        }
    }

    println!("{} tiles ends up being energized", visited.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input();
    let (height, width) = (map.len(), map[0].len());
    let mut max_energy = 0;

    for start_line in 0..height {
        let mut cycle_detector: HashSet<((usize, usize), Direction)> = HashSet::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut beams = vec![Beam::new((start_line, 0), Right)];

        while let Some(beam) = beams.pop() {
            if !cycle_detector.insert((beam.position, beam.direction)) {
                continue;
            }
            visited.insert(beam.position);
            let moved_beams = beam.tile_effect(&map);
            if let Some(beam) = moved_beams.0 {
                beams.push(beam);
            }
            if let Some(beam) = moved_beams.1 {
                beams.push(beam);
            }
        }
        max_energy = max_energy.max(visited.len());
        cycle_detector.clear();
        visited.clear();
        beams.push(Beam::new((start_line, width - 1), Left));
        while let Some(beam) = beams.pop() {
            if !cycle_detector.insert((beam.position, beam.direction)) {
                continue;
            }
            visited.insert(beam.position);
            let moved_beams = beam.tile_effect(&map);
            if let Some(beam) = moved_beams.0 {
                beams.push(beam);
            }
            if let Some(beam) = moved_beams.1 {
                beams.push(beam);
            }
        }
        max_energy = max_energy.max(visited.len());
    }

    for start_col in 0..width {
        let mut cycle_detector: HashSet<((usize, usize), Direction)> = HashSet::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut beams = vec![Beam::new((0, start_col), Down)];

        while let Some(beam) = beams.pop() {
            if !cycle_detector.insert((beam.position, beam.direction)) {
                continue;
            }
            visited.insert(beam.position);
            let moved_beams = beam.tile_effect(&map);
            if let Some(beam) = moved_beams.0 {
                beams.push(beam);
            }
            if let Some(beam) = moved_beams.1 {
                beams.push(beam);
            }
        }
        max_energy = max_energy.max(visited.len());
        cycle_detector.clear();
        visited.clear();
        beams.push(Beam::new((height - 1, start_col), Up));
        while let Some(beam) = beams.pop() {
            if !cycle_detector.insert((beam.position, beam.direction)) {
                continue;
            }
            visited.insert(beam.position);
            let moved_beams = beam.tile_effect(&map);
            if let Some(beam) = moved_beams.0 {
                beams.push(beam);
            }
            if let Some(beam) = moved_beams.1 {
                beams.push(beam);
            }
        }
        max_energy = max_energy.max(visited.len());
    }

    println!("{} tiles ends up being energized", max_energy);
    Ok(())
}
