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
    fn move_beam(mut self, map: &Vec<Vec<char>>) -> (Option<Self>, Option<Self>) {
        let (height, width) = (map.len(), map[0].len());
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
            let other_position = self.position;

            match map[self.position.0][self.position.1] {
                '.' => (Some(self), None),
                '-' => {
                    if self.direction == Up || self.direction == Down {
                        self.direction = Left;
                        (Some(self), Some(Self::new(other_position, Right)))
                    } else {
                        (Some(self), None)
                    }
                }
                '|' => {
                    if self.direction == Left || self.direction == Right {
                        self.direction = Up;
                        (Some(self), Some(Self::new(other_position, Down)))
                    } else {
                        (Some(self), None)
                    }
                }
                '/' => {
                    self.direction = match self.direction {
                        Up => Right,
                        Right => Up,
                        Down => Left,
                        Left => Down,
                    };
                    (Some(self), None)
                }
                '\\' => {
                    self.direction = match self.direction {
                        Up => Left,
                        Right => Down,
                        Down => Right,
                        Left => Up,
                    };
                    (Some(self), None)
                }
                _ => unreachable!(),
            }
        } else {
            (None, None)
        }
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
    let first_dir = match map[0][0] {
        '.' | '-' => Right,
        '\\' | '|' => Down,
        _ => unreachable!(),
    };
    let mut beams = vec![Beam::new((0, 0), first_dir)];

    while let Some(beam) = beams.pop() {
        if !cycle_detector.insert((beam.position, beam.direction)) {
            continue;
        }
        visited.insert(beam.position);
        let moved_beams = beam.move_beam(&map);
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
    unimplemented!("Star 2 not ready");
}
