use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SEBend,
    SWBend,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;
use Tile::*;

fn get_input() -> (Vec<Vec<Tile>>, (usize, usize)) {
    let input = fs::read_to_string(Path::new("./input/day10.input"))
        .expect("Something went wrong with the input");
    let mut start_coord = (0, 0);
    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(|(col_no, elem)| match elem {
                    '|' => Vertical,
                    '-' => Horizontal,
                    'L' => NEBend,
                    'J' => NWBend,
                    '7' => SWBend,
                    'F' => SEBend,
                    '.' => Ground,
                    'S' => {
                        start_coord = (line_no, col_no);
                        Start
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    (map, start_coord)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (map, start) = get_input();

    let mut ghosts = vec![
        (start, North, 0, HashSet::new()),
        (start, East, 0, HashSet::new()),
        (start, South, 0, HashSet::new()),
        (start, West, 0, HashSet::new()),
    ];

    let mut loop_len = 0;

    while let Some((position, direction, steps, mut visited)) = ghosts.pop() {
        if map[position.0][position.1] == Start && steps != 0 {
            loop_len = loop_len.max(steps);
            break;
        }

        if !visited.insert(position) {
            continue;
        }

        match direction {
            North => {
                let pos = (position.0.saturating_sub(1), position.1);
                if pos != position {
                    let tile = &map[pos.0][pos.1];
                    if *tile != Ground {
                        ghosts.push((pos, direction, steps + 1, visited.clone()));
                        if *tile == SEBend {
                            ghosts.push((pos, East, steps + 1, visited.clone()));
                        }
                        if *tile == SWBend {
                            ghosts.push((pos, West, steps + 1, visited.clone()));
                        }
                    }
                }
            }
            East => {
                let pos = (position.0, position.1 + 1);
                if let Some(tile) = map[pos.0].get(pos.1) {
                    if *tile != Ground {
                        ghosts.push((pos, direction, steps + 1, visited.clone()));
                        if *tile == NWBend {
                            ghosts.push((pos, North, steps + 1, visited.clone()));
                        }
                        if *tile == SWBend {
                            ghosts.push((pos, South, steps + 1, visited.clone()));
                        }
                    }
                }
            }
            South => {
                let pos = (position.0 + 1, position.1);
                if let Some(line) = map.get(pos.0) {
                    let tile = &line[pos.1];
                    ghosts.push((pos, direction, steps + 1, visited.clone()));
                    if *tile == NEBend {
                        ghosts.push((pos, East, steps + 1, visited.clone()));
                    }
                    if *tile == NWBend {
                        ghosts.push((pos, West, steps + 1, visited.clone()));
                    }
                }
            }
            West => {
                let pos = (position.0, position.1.saturating_sub(1));
                if pos != position {
                    let tile = &map[pos.0][pos.1];
                    if *tile != Ground {
                        ghosts.push((pos, direction, steps + 1, visited.clone()));
                        if *tile == SEBend {
                            ghosts.push((pos, South, steps + 1, visited.clone()));
                        }
                        if *tile == NEBend {
                            ghosts.push((pos, North, steps + 1, visited.clone()));
                        }
                    }
                }
            }
        }
    }

    println!("Furthest point on the loop is {} steps away", loop_len / 2);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
