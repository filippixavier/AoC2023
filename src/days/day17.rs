use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

type HeatMap = Vec<Vec<usize>>;
type Coordinate = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

fn get_input() -> HeatMap {
    let input = fs::read_to_string(Path::new("./input/day17.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|elem| elem.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn get_neighbors(
    coordinate: Coordinate,
    direction: Direction,
    width: usize,
    height: usize,
) -> Vec<(Coordinate, Direction)> {
    let mut neighbors = vec![];
    if coordinate.0 > 0 && direction != Down {
        neighbors.push(((coordinate.0 - 1, coordinate.1), Up));
    }
    if coordinate.1 > 0 && direction != Right {
        neighbors.push(((coordinate.0, coordinate.1 - 1), Left));
    }
    if coordinate.0 < height - 1 && direction != Up {
        neighbors.push(((coordinate.0 + 1, coordinate.1), Down));
    }
    if coordinate.1 < width - 1 && direction != Left {
        neighbors.push(((coordinate.0, coordinate.1 + 1), Right));
    }

    neighbors
}

fn least_heat(map: &HeatMap, start: Coordinate, end: Coordinate, max_straight: usize) -> usize {
    let (height, width) = (map.len(), map[0].len());
    let mut to_visit: Vec<_> = get_neighbors(start, Right, width, height)
        .into_iter()
        .map(|(coord, dir)| (coord, map[coord.0][coord.1], dir, 0))
        .collect();
    let mut cumulated_heatmap: HashMap<(Coordinate, Direction, usize), usize> = to_visit
        .iter()
        .map(|tuple| ((tuple.0, tuple.2, tuple.3), tuple.1))
        .collect();

    while !to_visit.is_empty() {
        to_visit.sort_unstable_by(|tuple_a, tuple_b| tuple_b.1.cmp(&tuple_a.1));
        let (coord, total_heat, dir, steps) = to_visit.pop().unwrap();

        if coord == end {
            break;
        }

        for (next_pos, next_dir) in get_neighbors(coord, dir, width, height) {
            if next_dir == dir && steps + 1 == max_straight {
                continue;
            }
            let next_heat = total_heat + map[next_pos.0][next_pos.1];
            let next_step = if next_dir == dir { steps + 1 } else { 0 };
            let previous_heat = cumulated_heatmap
                .entry((next_pos, next_dir, next_step))
                .or_insert(usize::MAX);
            if *previous_heat > next_heat {
                *previous_heat = next_heat;
                to_visit.push((next_pos, next_heat, next_dir, next_step));
            }
        }
    }

    cumulated_heatmap
        .into_iter()
        .filter(|(key_tuple, _)| key_tuple.0 == end)
        .map(|(_, value)| value)
        .min()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input();
    let minimal_heat = least_heat(&map, (0, 0), (map.len() - 1, map[0].len() - 1), 3);
    println!("{}", minimal_heat);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
