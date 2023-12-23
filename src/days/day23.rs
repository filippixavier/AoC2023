use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Flat,
    LeftSlope,
    RightSlope,
    UpSlope,
    DownSlope,
    Rock,
}

use Tile::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

use Facing::*;

#[derive(Debug, Clone, Copy)]
struct Hiker {
    position: Coordinate,
    facing: Facing,
    step_count: usize,
}

fn get_input() -> Vec<Vec<Tile>> {
    let input = fs::read_to_string(Path::new("./input/day23.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| match tile {
                    '#' => Rock,
                    '.' => Flat,
                    '>' => RightSlope,
                    '<' => LeftSlope,
                    'v' => DownSlope,
                    '^' => UpSlope, // never happen neither in test or real input
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let hike_map = get_input();

    let start_row = hike_map[0]
        .iter()
        .position(|tile| *tile == Flat)
        .unwrap_or(0);
    let mut hikers: Vec<(Hiker, HashSet<Coordinate>)> = vec![(
        Hiker {
            facing: Down,
            position: Coordinate { x: 0, y: start_row },
            step_count: 0,
        },
        HashSet::new(),
    )];

    let mut max_step = 0;

    while let Some((mut hiker, mut visited_coordinates)) = hikers.pop() {
        loop {
            let next_coordinate = match hiker.facing {
                Up => Coordinate {
                    x: hiker.position.x - 1,
                    y: hiker.position.y,
                },
                Left => Coordinate {
                    x: hiker.position.x,
                    y: hiker.position.y - 1,
                },
                Down => Coordinate {
                    x: hiker.position.x + 1,
                    y: hiker.position.y,
                },
                Right => Coordinate {
                    x: hiker.position.x,
                    y: hiker.position.y + 1,
                },
            };
            let tile = hike_map[next_coordinate.x][next_coordinate.y];
            let mut forced = false;
            match tile {
                Rock => break,
                LeftSlope => {
                    if hiker.facing == Right {
                        break;
                    }
                    forced = true;
                    hiker.facing = Left
                }
                RightSlope => {
                    if hiker.facing == Left {
                        break;
                    }
                    forced = true;
                    hiker.facing = Right;
                }
                DownSlope => {
                    if hiker.facing == Up {
                        break;
                    }
                    forced = true;
                    hiker.facing = Down;
                }
                UpSlope => {
                    if hiker.facing == Down {
                        break;
                    }
                    forced = true;
                    hiker.facing = Up;
                }
                Flat => {}
            }

            if !visited_coordinates.insert(next_coordinate) {
                break;
            }
            hiker.position = next_coordinate;
            hiker.step_count += 1;

            if hiker.position.x == hike_map.len() - 1 {
                max_step = max_step.max(hiker.step_count);
                break;
            }

            if !forced {
                let next_facing: Vec<Facing> = vec![Up, Down, Left, Right]
                    .into_iter()
                    .filter(|&face| match face {
                        Up => {
                            hiker.facing != Down
                                && hike_map[hiker.position.x - 1][hiker.position.y] != Rock
                                && hike_map[hiker.position.x - 1][hiker.position.y] != DownSlope
                        }
                        Down => {
                            hiker.facing != Up
                                && hike_map[hiker.position.x + 1][hiker.position.y] != Rock
                                && hike_map[hiker.position.x + 1][hiker.position.y] != UpSlope
                        }
                        Left => {
                            hiker.facing != Right
                                && hike_map[hiker.position.x][hiker.position.y - 1] != Rock
                                && hike_map[hiker.position.x][hiker.position.y - 1] != RightSlope
                        }
                        Right => {
                            hiker.facing != Left
                                && hike_map[hiker.position.x][hiker.position.y + 1] != Rock
                                && hike_map[hiker.position.x][hiker.position.y + 1] != LeftSlope
                        }
                    })
                    .collect();
                hiker.facing = next_facing[0];
                for other_facing in next_facing.into_iter().skip(1) {
                    let mut other_hiker = hiker;
                    other_hiker.facing = other_facing;
                    hikers.push((other_hiker, visited_coordinates.clone()));
                }
            }
        }
    }
    println!("🏔️ The longest, most scenic view is {} steps long", max_step);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let hike_map: Vec<Vec<Tile>> = get_input()
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|tile| if tile == Rock { Rock } else { Flat })
                .collect()
        })
        .collect();

    let start_row = hike_map[0]
        .iter()
        .position(|tile| *tile == Flat)
        .unwrap_or(0);
    let mut hikers: Vec<(Hiker, HashSet<Coordinate>)> = vec![(
        Hiker {
            facing: Down,
            position: Coordinate { x: 0, y: start_row },
            step_count: 0,
        },
        HashSet::new(),
    )];

    let mut max_step = 0;

    while let Some((mut hiker, mut visited_coordinates)) = hikers.pop() {
        loop {
            let next_coordinate = match hiker.facing {
                Up => Coordinate {
                    x: hiker.position.x - 1,
                    y: hiker.position.y,
                },
                Left => Coordinate {
                    x: hiker.position.x,
                    y: hiker.position.y - 1,
                },
                Down => Coordinate {
                    x: hiker.position.x + 1,
                    y: hiker.position.y,
                },
                Right => Coordinate {
                    x: hiker.position.x,
                    y: hiker.position.y + 1,
                },
            };
            let tile = hike_map[next_coordinate.x][next_coordinate.y];
            if tile == Rock {
                break;
            }

            if !visited_coordinates.insert(next_coordinate) {
                break;
            }
            hiker.position = next_coordinate;
            hiker.step_count += 1;

            if hiker.position.x == hike_map.len() - 1 {
                max_step = max_step.max(hiker.step_count);
                break;
            }

            let next_facing: Vec<Facing> = vec![Up, Down, Left, Right]
                .into_iter()
                .filter(|&face| match face {
                    Up => {
                        hiker.facing != Down
                            && hike_map[hiker.position.x - 1][hiker.position.y] != Rock
                    }
                    Down => {
                        hiker.facing != Up
                            && hike_map[hiker.position.x + 1][hiker.position.y] != Rock
                    }
                    Left => {
                        hiker.facing != Right
                            && hike_map[hiker.position.x][hiker.position.y - 1] != Rock
                    }
                    Right => {
                        hiker.facing != Left
                            && hike_map[hiker.position.x][hiker.position.y + 1] != Rock
                    }
                })
                .collect();
            hiker.facing = next_facing[0];
            for other_facing in next_facing.into_iter().skip(1) {
                let mut other_hiker = hiker;
                other_hiker.facing = other_facing;
                hikers.push((other_hiker, visited_coordinates.clone()));
            }
        }
    }
    println!("Since the trails are surprisingly dry, 🏔️ The longest, most scenic view is, in fact, {} steps long", max_step);
    Ok(())
}
