use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

type StreamData = (Vec<char>, Vec<usize>);
fn get_input() -> Vec<StreamData> {
    let input = fs::read_to_string(Path::new("./input/day12.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().chars().collect(),
                parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(str::parse::<usize>)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            )
        })
        .collect_vec()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut total_combinations = 0;

    for (line, checksum) in input {
        let total: usize = checksum.iter().sum();
        let checksum_value = checksum.iter().map(|elem| elem.to_string()).join(",");
        let missing_in_input: usize = total
            - line
                .iter()
                .fold(0, |acc, &elem| if elem == '#' { acc + 1 } else { acc });
        let positions_to_fill = line
            .iter()
            .enumerate()
            .filter_map(|(pos, &value)| if value == '?' { Some(pos) } else { None })
            .collect_vec();

        for perm in positions_to_fill.iter().combinations(missing_in_input) {
            let mut cloned = line.clone();
            for pos in perm {
                cloned[*pos] = '#';
            }
            let mut tentative_checksum = vec![];
            for (is_valid, group) in &cloned.into_iter().group_by(|&key| key == '#') {
                if is_valid {
                    tentative_checksum.push(group.count());
                }
            }
            if tentative_checksum
                .into_iter()
                .map(|elem| elem.to_string())
                .join(",")
                == checksum_value
            {
                total_combinations += 1;
            }
        }
    }

    println!("{:?}", total_combinations);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
