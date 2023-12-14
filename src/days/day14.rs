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

    println!(
        "The total load of the north support beam is {} ",
        max_weight
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
