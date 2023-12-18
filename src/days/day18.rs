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

fn get_input() -> Vec<(Direction, usize, (usize, Direction))> {
    let reg = Regex::new(r"(\w) (\d+) \(#(.{5})(\d)\)").unwrap();
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
            let backup_steps = usize::from_str_radix(&cap[3], 16).unwrap_or(0);
            let backup_dir = match usize::from_str_radix(&cap[4], 16).unwrap_or(0) {
                0 => Right,
                1 => Down,
                2 => Left,
                3 => Up,
                _ => unreachable!(),
            };
            (dir, steps, (backup_steps, backup_dir))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    start: Coordinate,
    end: Coordinate,
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut pos = Coordinate { x: 0, y: 0 };
    let mut digged = 0;
    let mut verticals: Vec<Vector> = vec![];
    let mut horizontals: Vec<Vector> = vec![];

    // Fetch all horizontals and verticals vectors
    for (_, _, (steps, dir)) in input {
        pos = match dir {
            Up => {
                let next_pos = Coordinate {
                    x: pos.x - steps as isize,
                    y: pos.y,
                };
                verticals.push(Vector {
                    start: next_pos,
                    end: pos,
                });
                next_pos
            }
            Down => {
                let next_pos = Coordinate {
                    x: pos.x + steps as isize,
                    y: pos.y,
                };
                verticals.push(Vector {
                    start: pos,
                    end: next_pos,
                });
                next_pos
            }
            Left => {
                let next_pos = Coordinate {
                    x: pos.x,
                    y: pos.y - steps as isize,
                };
                horizontals.push(Vector {
                    start: next_pos,
                    end: pos,
                });
                next_pos
            }
            Right => {
                let next_pos = Coordinate {
                    x: pos.x,
                    y: pos.y + steps as isize,
                };
                horizontals.push(Vector {
                    start: pos,
                    end: next_pos,
                });
                next_pos
            }
        };
    }

    while !horizontals.is_empty() {
        horizontals.sort_unstable_by(|vec_a, vec_b| vec_b.start.x.cmp(&vec_a.start.x));
        let mut current = horizontals.pop().unwrap();
        let width = current.end.y - current.start.y + 1;

        // Get the two vertical element at each side of the horizontal one
        let sub_vert: Vec<_> = verticals
            .iter()
            .cloned()
            .filter(|vert| vert.start == current.start || vert.start == current.end)
            .collect();
        // No vertical elements means we just add the width and continue
        if sub_vert.is_empty() {
            digged += width;
            continue;
        }
        verticals.retain(|elem| !sub_vert.contains(elem));
        let (mut left_vert, mut right_vert) = (sub_vert[0], sub_vert[1]);
        // We don't get below the smallest vertical vector
        let mut min_down = left_vert.end.x.min(right_vert.end.x);
        // Check if there is an horizontal vector before
        min_down = horizontals
            .iter()
            .filter_map(|elem| {
                if elem.start.y <= current.end.y
                    && elem.end.y >= current.start.y
                    && elem.start.x > current.start.x
                    && elem.start.x <= min_down
                {
                    Some(elem.start.x)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(min_down);

        let height = min_down - current.start.x;

        // Move the vector down
        current.start.x = min_down;
        current.end.x = min_down;

        digged += width * height;

        // Shrink verticals vectors accordingly and add them if dist != 0
        left_vert.start.x = min_down;
        right_vert.start.x = min_down;

        if left_vert.start != left_vert.end {
            verticals.push(left_vert);
        }

        if right_vert.start != right_vert.end {
            verticals.push(right_vert);
        }
        // Get every horizontal vectors on the same height that are connected to our horizontal vector
        let mut connected_h: Vec<_> = horizontals
            .iter()
            .cloned()
            .filter(|elem| {
                elem.start.x == min_down
                    && elem.start.y <= current.end.y
                    && elem.end.y >= current.start.y
            })
            .collect();
        horizontals.retain(|elem| !connected_h.contains(elem));

        connected_h.sort_unstable_by(|vec_a, vec_b| vec_a.start.y.cmp(&vec_b.start.y));
        // Check every horizontal vectors to merge, or to split current vector
        for connected in connected_h {
            if connected.end == current.start {
                current.start = connected.start;
            } else if connected.start == current.start {
                digged += connected.end.y - connected.start.y;
                current.start = connected.end;
            } else if connected.start == current.end {
                current.end = connected.end;
            } else if connected.end == current.end {
                digged += connected.end.y - connected.start.y;
                current.end = connected.start;
            } else {
                horizontals.push(Vector {
                    start: current.start,
                    end: connected.start,
                });
                digged += connected.end.y - connected.start.y - 1; // Taking overlapping into account
                current.start = connected.end;
            }
        }
        horizontals.push(current);
    }
    println!(
        "⛏️ Using the *real* instructions, the lagoon can hold up to {} cubic meters of lava",
        digged
    );
    Ok(())
}
