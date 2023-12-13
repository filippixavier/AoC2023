use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

use itertools::Itertools;
use Tile::*;

type Field = Vec<Vec<Tile>>;

fn get_input() -> Vec<Field> {
    let input = fs::read_to_string(Path::new("./input/day13.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .split("\r\n\r\n")
        .map(|field| {
            field
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|tile| match tile {
                            '.' => Ash,
                            '#' => Rock,
                            _ => unreachable!(),
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}
pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let fields = get_input();
    let mut horizontal_mirrors = 0usize;
    let mut vertical_mirrors = 0usize;

    'field: for field in fields {
        let width = field[0].len();
        let height = field.len();
        // horizontal flip detection
        for i in 1..height {
            let up = field.iter().take(i).rev();
            let down = field.iter().skip(i);
            if up.zip(down).all(|(up_line, down_line)| {
                up_line
                    .iter()
                    .zip(down_line.iter())
                    .all(|(up_tile, down_tile)| up_tile == down_tile)
            }) {
                horizontal_mirrors += i;
                continue 'field;
            }
        }
        // vertical flip detection
        for j in 1..width {
            if field.iter().all(|line| {
                let left = line.iter().take(j).rev();
                let right = line.iter().skip(j);
                left.zip(right)
                    .all(|(left_tile, right_tile)| left_tile == right_tile)
            }) {
                vertical_mirrors += j;
                continue 'field;
            }
        }
    }
    println!(
        "Value obtained after summarizing all notes is: {}",
        vertical_mirrors + 100 * horizontal_mirrors
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let fields = get_input();
    let mut horizontal_mirrors = 0usize;
    let mut vertical_mirrors = 0usize;

    'field: for field in fields {
        let width = field[0].len();
        let height = field.len();
        // horizontal flip detection with exactly 1 error
        for i in 1..height {
            let up = field.iter().take(i).rev();
            let down = field.iter().skip(i);
            let mut error_counter = 0;
            for (line_up, line_down) in up.zip(down) {
                error_counter +=
                    line_up
                        .iter()
                        .zip(line_down.iter())
                        .fold(
                            0,
                            |acc, (up_tile, down_tile)| {
                                if up_tile != down_tile {
                                    acc + 1
                                } else {
                                    acc
                                }
                            },
                        );
                if error_counter > 1 {
                    break;
                }
            }
            if error_counter == 1 {
                horizontal_mirrors += i;
                continue 'field;
            }
        }
        // vertical flip detection
        for j in 1..width {
            let mut error_counter = 0;
            for line in field.iter() {
                let left = line.iter().take(j).rev();
                let right = line.iter().skip(j);
                error_counter += left.zip(right).fold(0, |acc, (left_tile, right_tile)| {
                    if left_tile != right_tile {
                        acc + 1
                    } else {
                        acc
                    }
                });
                if error_counter > 1 {
                    break;
                }
            }
            if error_counter == 1 {
                vertical_mirrors += j;
                continue 'field;
            }
        }
    }
    println!(
        "After fixing the smudge, the value obtained after summarizing all notes is: {}",
        vertical_mirrors + 100 * horizontal_mirrors
    );
    Ok(())
}
