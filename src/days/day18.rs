use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SEBend,
    SWBend,
}

use Direction::*;
use Tile::*;

fn get_input() -> Vec<(Direction, usize, usize)> {
    let reg = Regex::new(r"(\w) (\d+) \(#(.*)\)").unwrap();
    let input = fs::read_to_string(Path::new("./input/day18.input"))
        .expect("Something went wrong with the input");

    reg.captures_iter(&input)
        .map(|cap| {
            let dir = match &cap[1] {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => unreachable!(),
            };
            let steps = cap[2].parse::<usize>().unwrap_or(0);
            let color = usize::from_str_radix(&cap[3], 16).unwrap_or(0);
            (dir, steps, color)
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut digged = 0;
    let mut pos = (0, 0);
    let mut dig_map: HashMap<(isize, isize), (Tile, Direction)> = HashMap::new();

    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);

    for (dir, steps, _) in input {
        let (movement, tile) = match dir {
            Up => ((-1, 0), Vertical),
            Down => ((1, 0), Vertical),
            Left => ((0, -1), Horizontal),
            Right => ((0, 1), Horizontal),
        };
        dig_map
            .entry(pos)
            .and_modify(|(previous_tile, previous_dir)| {
                *previous_tile = match *previous_dir {
                    Up => {
                        if dir == Left {
                            SWBend
                        } else {
                            SEBend
                        }
                    }
                    Down => {
                        if dir == Left {
                            NWBend
                        } else {
                            NEBend
                        }
                    }
                    Left => {
                        if dir == Up {
                            NEBend
                        } else {
                            SEBend
                        }
                    }
                    Right => {
                        if dir == Up {
                            NWBend
                        } else {
                            SWBend
                        }
                    }
                }
            })
            .or_insert((tile, dir));
        for _ in 0..steps {
            pos = (pos.0 + movement.0, pos.1 + movement.1);
            dig_map.insert(pos, (tile, dir));
        }
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
    }

    for line in min_x..=max_x {
        let mut digging = false;
        let (mut north, mut south) = (false, false);
        for col in min_y..=max_y {
            if let Some((tile, _)) = dig_map.get(&(line, col)) {
                digged += 1;
                match tile {
                    SWBend | SEBend => {
                        south = !south;
                        if north && south {
                            north = false;
                            south = false;
                            digging = !digging;
                        }
                    }
                    NWBend | NEBend => {
                        north = !north;
                        if north && south {
                            north = false;
                            south = false;
                            digging = !digging
                        }
                    }
                    Vertical => {
                        digging = !digging;
                        north = false;
                        south = false;
                    }
                    Horizontal => {}
                }
            } else if digging {
                digged += 1;
            }
        }
    }

    println!(
        "⛏️ After digging, the lagoon will hold {} cubic meters of lava",
        digged
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
