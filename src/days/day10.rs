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
        (start, North, 0),
        (start, East, 0),
        (start, South, 0),
        (start, West, 0),
    ];

    let mut loop_len = 0;

    while let Some((position, direction, steps)) = ghosts.pop() {
        if map[position.0][position.1] == Start && steps != 0 {
            loop_len = loop_len.max(steps);
            continue;
        }

        match direction {
            North => {
                let pos = (position.0.saturating_sub(1), position.1);
                if pos != position {
                    let tile = &map[pos.0][pos.1];
                    if *tile != Ground {
                        if *tile == SEBend {
                            ghosts.push((pos, East, steps + 1));
                        }
                        if *tile == SWBend {
                            ghosts.push((pos, West, steps + 1));
                        }
                        if *tile == Vertical || *tile == Start {
                            ghosts.push((pos, direction, steps + 1));
                        }
                    }
                }
            }
            East => {
                let pos = (position.0, position.1 + 1);
                if let Some(tile) = map[pos.0].get(pos.1) {
                    if *tile != Ground {
                        if *tile == NWBend {
                            ghosts.push((pos, North, steps + 1));
                        }
                        if *tile == SWBend {
                            ghosts.push((pos, South, steps + 1));
                        }
                        if *tile == Horizontal || *tile == Start {
                            ghosts.push((pos, direction, steps + 1));
                        }
                    }
                }
            }
            South => {
                let pos = (position.0 + 1, position.1);
                if let Some(line) = map.get(pos.0) {
                    let tile = &line[pos.1];
                    if *tile == NEBend {
                        ghosts.push((pos, East, steps + 1));
                    }
                    if *tile == NWBend {
                        ghosts.push((pos, West, steps + 1));
                    }
                    if *tile == Vertical || *tile == Start {
                        ghosts.push((pos, direction, steps + 1));
                    }
                }
            }
            West => {
                let pos = (position.0, position.1.saturating_sub(1));
                if pos != position {
                    let tile = &map[pos.0][pos.1];
                    if *tile != Ground {
                        if *tile == SEBend {
                            ghosts.push((pos, South, steps + 1));
                        }
                        if *tile == NEBend {
                            ghosts.push((pos, North, steps + 1));
                        }
                        if *tile == Horizontal || *tile == Start {
                            ghosts.push((pos, direction, steps + 1));
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
    let (mut map, start) = get_input();

    let mut ghosts = vec![
        (start, North, vec![]),
        (start, East, vec![]),
        (start, South, vec![]),
        (start, West, vec![]),
    ];

    let mut loop_path = vec![];

    // Find greatest loop
    while let Some((position, direction, mut path)) = ghosts.pop() {
        if map[position.0][position.1] == Start && !path.is_empty() {
            loop_path = if path.len() > loop_path.len() {
                path
            } else {
                loop_path
            };
            continue;
        }

        path.push(position);

        match direction {
            North => {
                let pos = (position.0.saturating_sub(1), position.1);
                if pos != position {
                    let tile = &map[pos.0][pos.1];
                    if *tile != Ground {
                        if *tile == SEBend {
                            ghosts.push((pos, East, path.clone()));
                        }
                        if *tile == SWBend {
                            ghosts.push((pos, West, path.clone()));
                        }
                        if *tile == Vertical || *tile == Start {
                            ghosts.push((pos, direction, path.clone()));
                        }
                    }
                }
            }
            East => {
                let pos = (position.0, position.1 + 1);
                if let Some(tile) = map[pos.0].get(pos.1) {
                    if *tile != Ground {
                        if *tile == NWBend {
                            ghosts.push((pos, North, path.clone()));
                        }
                        if *tile == SWBend {
                            ghosts.push((pos, South, path.clone()));
                        }
                        if *tile == Horizontal || *tile == Start {
                            ghosts.push((pos, direction, path.clone()));
                        }
                    }
                }
            }
            South => {
                let pos = (position.0 + 1, position.1);
                if let Some(line) = map.get(pos.0) {
                    let tile = &line[pos.1];
                    if *tile == NEBend {
                        ghosts.push((pos, East, path.clone()));
                    }
                    if *tile == NWBend {
                        ghosts.push((pos, West, path.clone()));
                    }
                    if *tile == Vertical || *tile == Start {
                        ghosts.push((pos, direction, path.clone()));
                    }
                }
            }
            West => {
                let pos = (position.0, position.1.saturating_sub(1));
                if pos != position {
                    let tile = &map[pos.0][pos.1];
                    if *tile != Ground {
                        if *tile == SEBend {
                            ghosts.push((pos, South, path.clone()));
                        }
                        if *tile == NEBend {
                            ghosts.push((pos, North, path.clone()));
                        }
                    }
                    if *tile == Horizontal || *tile == Start {
                        ghosts.push((pos, direction, path.clone()));
                    }
                }
            }
        }
    }

    let (first_tile, last_tile) = (loop_path[1], *loop_path.last().unwrap());
    let start_tile = if first_tile.0 == last_tile.0 {
        Horizontal
    } else if first_tile.1 == last_tile.1 {
        Vertical
    } else if start.0 == first_tile.0 {
        if first_tile.0 < last_tile.0 && first_tile.1 < last_tile.1 {
            SWBend
        } else if first_tile.0 < last_tile.0 && first_tile.1 > last_tile.1 {
            SEBend
        } else if first_tile.0 > last_tile.0 && first_tile.1 < last_tile.1 {
            NWBend
        } else {
            NEBend
        }
    } else if first_tile.0 < last_tile.0 && first_tile.1 < last_tile.1 {
        NEBend
    } else if first_tile.0 < last_tile.0 && first_tile.1 > last_tile.1 {
        NWBend
    } else if first_tile.0 > last_tile.0 && first_tile.1 < last_tile.1 {
        SEBend
    } else {
        SWBend
    };

    map[start.0][start.1] = start_tile;

    let mut insiders = vec![];

    // Accidentally saw this solution on the reddit after solving day 11
    // I didn't read the explanation, but if I suppose that because it's a loop, I pass over the loop an even number of time
    let count_internal = map
        .iter()
        .enumerate()
        .map(|(line_no, line)| {
            let mut north = false;
            let mut south = false;
            let mut inside = false;
            line.iter()
                .enumerate()
                .filter(|&(col_no, tile)| {
                    let in_loop = loop_path.contains(&(line_no, col_no));
                    let is_inside = match tile {
                        Ground => inside,
                        Horizontal => !in_loop && inside,
                        Vertical => {
                            if in_loop {
                                inside = !inside;
                                north = false;
                                south = false;
                                false
                            } else {
                                inside
                            }
                        }
                        NEBend | NWBend => {
                            if in_loop {
                                north = !north;
                                if north && south {
                                    inside = !inside;
                                    north = false;
                                    south = false;
                                }
                                false
                            } else {
                                inside
                            }
                        }
                        SWBend | SEBend => {
                            if in_loop {
                                south = !south;
                                if north && south {
                                    inside = !inside;
                                    north = false;
                                    south = false;
                                }
                                false
                            } else {
                                inside
                            }
                        }
                        _ => unreachable!(),
                    };
                    if is_inside {
                        insiders.push((line_no, col_no));
                    }
                    is_inside
                })
                .count()
        })
        .sum::<usize>();
    println!("There are {} elements inside the loop", count_internal);
    Ok(())
}
