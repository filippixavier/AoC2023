use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(Path::new("./input/day3.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let schematic = get_input();
    let (rows, cols) = (schematic.len(), schematic[0].len());
    let mut total = 0;

    for i in 0..rows {
        let (mut num, mut has_symbol) = (0, false);
        for j in 0..cols {
            let token = schematic[i][j];
            if token.is_ascii_digit() {
                num = num * 10 + token.to_digit(10).unwrap();
                for offset_i in -1..=1 {
                    let look_i = i as isize + offset_i;
                    if look_i < 0 || look_i >= rows as isize {
                        continue;
                    }
                    for offset_j in -1..=1 {
                        let look_j = j as isize + offset_j;
                        if look_j < 0 || look_j >= cols as isize {
                            continue;
                        }
                        let neighbor = schematic[look_i as usize][look_j as usize];
                        if neighbor != '.' && !neighbor.is_ascii_digit() {
                            has_symbol = true;
                            break;
                        }
                    }
                }
            } else if token == '.' {
                if has_symbol {
                    total += num;
                }
                num = 0;
                has_symbol = false;
            } else {
                total += num;
                has_symbol = false;
                num = 0;
            }
        }
        if has_symbol {
            total += num;
        }
    }
    println!("The sum of valid parts is: {}", total);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let schematic = get_input();
    let (rows, cols) = (schematic.len(), schematic[0].len());
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..rows {
        let (mut num, mut gear) = (0, None);
        for j in 0..cols {
            let token = schematic[i][j];
            if token.is_ascii_digit() {
                num = num * 10 + token.to_digit(10).unwrap();
                for offset_i in -1..=1 {
                    let look_i = i as isize + offset_i;
                    if look_i < 0 || look_i >= rows as isize {
                        continue;
                    }
                    for offset_j in -1..=1 {
                        let look_j = j as isize + offset_j;
                        if look_j < 0 || look_j >= cols as isize {
                            continue;
                        }
                        let neighbor = schematic[look_i as usize][look_j as usize];
                        if neighbor == '*' {
                            gear = Some((look_i as usize, look_j as usize));
                            break;
                        }
                    }
                }
            } else {
                if let Some(gear_coor) = gear {
                    gears
                        .entry(gear_coor)
                        .and_modify(|values| values.push(num))
                        .or_insert(vec![num]);
                }
                num = 0;
                gear = None;
            }
        }
        if let Some(gear_coor) = gear {
            gears
                .entry(gear_coor)
                .and_modify(|values| values.push(num))
                .or_insert(vec![num]);
        }
    }

    println!(
        "The sum of gears ratios is: {}",
        gears
            .values()
            .filter_map(|adjacents| {
                if adjacents.len() == 2 {
                    Some(adjacents[0] * adjacents[1])
                } else {
                    None
                }
            })
            .sum::<u32>()
    );
    Ok(())
}
