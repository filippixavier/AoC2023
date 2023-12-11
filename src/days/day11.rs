use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(Path::new("./input/day11.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|lines| lines.trim().chars().collect_vec())
        .collect_vec()
}

fn get_empties(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_cols = vec![];
    'outer: for i in 0..universe[0].len() {
        for line in universe {
            if line[i] == '#' {
                continue 'outer;
            }
        }
        empty_cols.push(i);
    }

    let empty_lines = universe
        .iter()
        .enumerate()
        .filter_map(|(line_no, line)| {
            if line.iter().all(|value| *value == '.') {
                Some(line_no)
            } else {
                None
            }
        })
        .collect_vec();
    (empty_lines, empty_cols)
}

fn expand_universe(universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (empty_lines, empty_cols) = get_empties(&universe);

    let expanded_cols = universe[0].len() + empty_cols.len();

    let mut expanded = vec![];

    for (line_no, line) in universe.iter().enumerate() {
        if empty_lines.contains(&line_no) {
            expanded.push(vec!['.'; expanded_cols]);
            expanded.push(vec!['.'; expanded_cols]);
        } else {
            let mut col = vec![];
            for (col_no, value) in line.iter().enumerate() {
                col.push(*value);
                if empty_cols.contains(&col_no) {
                    col.push(*value);
                }
            }
            expanded.push(col);
        }
    }

    expanded
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let universe = expand_universe(get_input());
    let galaxies = universe
        .iter()
        .enumerate()
        .filter_map(|(line_no, line)| {
            let coordinates = line
                .iter()
                .enumerate()
                .filter_map(|(col_no, value)| {
                    if *value == '.' {
                        None
                    } else {
                        Some((line_no, col_no))
                    }
                })
                .collect_vec();
            if coordinates.is_empty() {
                None
            } else {
                Some(coordinates)
            }
        })
        .flatten()
        .collect_vec();

    let shortests = galaxies
        .iter()
        .combinations(2)
        .map(|combo| {
            let (start, end) = (combo[0], combo[1]);
            start.0.max(end.0) - start.0.min(end.0) + start.1.max(end.1) - start.1.min(end.1)
        })
        .collect_vec();

    println!(
        "The sums of the shortest paths is {}",
        shortests.iter().sum::<usize>()
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let universe = get_input();
    let (empty_lines, empty_cols) = get_empties(&universe);
    let galaxies = universe
        .iter()
        .enumerate()
        .filter_map(|(line_no, line)| {
            let coordinates = line
                .iter()
                .enumerate()
                .filter_map(|(col_no, value)| {
                    if *value == '.' {
                        None
                    } else {
                        Some((line_no, col_no))
                    }
                })
                .collect_vec();
            if coordinates.is_empty() {
                None
            } else {
                Some(coordinates)
            }
        })
        .flatten()
        .collect_vec();

    let shortests = galaxies
        .iter()
        .combinations(2)
        .map(|combo| {
            let (start, end) = (combo[0], combo[1]);
            let (start_line, start_col, end_line, end_col) = (
                start.0.min(end.0),
                start.1.min(end.1),
                start.0.max(end.0),
                start.1.max(end.1),
            );
            let empty_lines_count = empty_lines
                .iter()
                .filter(|&&line_no| line_no > start_line && line_no < end_line)
                .count();
            let empty_cols_count = empty_cols
                .iter()
                .filter(|&&col_no| col_no > start_col && col_no < end_col)
                .count();
            let standard_dist = end_line - start_line + end_col - start_col;
            standard_dist + (empty_cols_count + empty_lines_count) * 999_999 // We've already counted the empty lines/cols once
        })
        .collect_vec();

    println!(
        "The sums of the shortest paths is {}",
        shortests.iter().sum::<usize>()
    );

    Ok(())
}
