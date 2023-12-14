use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Tile {
    Round,
    Square,
    Empty,
}

use Tile::*;

fn get_input() -> Vec<Vec<Tile>> {
    let input = fs::read_to_string(Path::new("./input/day14.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '.' => Empty,
                    'O' => Round,
                    '#' => Square,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let (height, width) = (input.len(), input[0].len());
    let mut max_weight = 0;

    for col in 0..width {
        let mut max_height = 0;
        for (line_no, line) in input.iter().enumerate() {
            if line[col] == Round {
                max_weight += height - max_height;
                max_height += 1;
            }
            if line[col] == Square {
                max_height = line_no + 1;
            }
        }
    }

    println!("The total load of the north support beam is {}", max_weight);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

use itertools::Itertools;
use Direction::*;

fn move_rocks(map: &mut Vec<Vec<Tile>>, direction: Direction) {
    let (height, width) = (map.len(), map[0].len());

    let (outer_loop, inner_loop, is_swap, is_reverse) = match direction {
        North => (
            (0..width).collect_vec(),
            (0..height).collect_vec(),
            true,
            false,
        ),
        East => (
            (0..height).collect_vec(),
            (0..width).rev().collect_vec(),
            false,
            true,
        ),
        South => (
            (0..width).collect_vec(),
            (0..height).rev().collect_vec(),
            true,
            true,
        ),
        West => (
            (0..height).collect_vec(),
            (0..width).collect_vec(),
            false,
            false,
        ),
    };

    for i in outer_loop {
        let mut fall_value = *inner_loop.first().unwrap();
        for &j in &inner_loop {
            if is_swap {
                if map[j][i] == Round {
                    map[j][i] = Empty;
                    map[fall_value][i] = Round;
                    fall_value = if is_reverse {
                        fall_value.saturating_sub(1)
                    } else {
                        fall_value + 1
                    };
                }

                if map[j][i] == Square {
                    fall_value = if is_reverse {
                        j.saturating_sub(1)
                    } else {
                        j + 1
                    };
                }
            } else {
                if map[i][j] == Round {
                    map[i][j] = Empty;
                    map[i][fall_value] = Round;
                    fall_value = if is_reverse {
                        fall_value.saturating_sub(1)
                    } else {
                        fall_value + 1
                    };
                }

                if map[i][j] == Square {
                    fall_value = if is_reverse {
                        j.saturating_sub(1)
                    } else {
                        j + 1
                    };
                }
            };
        }
    }
}

fn display_rocks(map: &Vec<Vec<Tile>>) -> String {
    let mut formatted = String::new();
    for i in map {
        for j in i {
            formatted += &format!(
                "{}",
                match j {
                    Round => 'O',
                    Square => '#',
                    Empty => '.',
                }
            )
            .to_string();
        }
        formatted += "\n";
    }
    formatted
}

fn get_weight(map: &Vec<Vec<Tile>>) -> usize {
    let height = map.len();
    map.iter()
        .enumerate()
        .map(|(line_no, line)| {
            (height - line_no) * line.iter().filter(|tile| **tile == Round).count()
        })
        .sum()
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut input = get_input();

    let mut repetition_detector = HashMap::new();
    let mut weights = vec![];
    let mut repetiting_weights = vec![];
    let mut repetition_start = 0;
    let directions = [North, West, South, East];
    for cycle in 0.. {
        for direction in directions.iter() {
            move_rocks(&mut input, *direction);
        }
        if let Some(previous_cycle) = repetition_detector.insert(display_rocks(&input), cycle) {
            repetiting_weights = weights.iter().skip(previous_cycle).collect();
            repetition_start = previous_cycle;
            break;
        }
        weights.push(get_weight(&input));
    }

    println!(
        "After 1 000 000 000 cycles, the total load of the north support beam is {}",
        repetiting_weights[((1_000_000_000 - repetition_start) % repetiting_weights.len()) - 1] // I have no idea why the off by one
    );

    Ok(())
}
