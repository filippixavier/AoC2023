use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> Vec<Vec<isize>> {
    let input = fs::read_to_string(Path::new("./input/day9.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<isize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect_vec()
}

fn predict_next(input: &[isize]) -> isize {
    if input.iter().all(|&value| value == 0) {
        0
    } else {
        let mut sub_input = vec![];
        for i in 1..input.len() {
            sub_input.push(input[i] - input[i - 1]);
        }
        input.last().unwrap() + predict_next(&sub_input)
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let readings = get_input();

    let predicted = readings
        .iter()
        .map(|input| predict_next(input))
        .collect_vec();

    println!(
        "The sum of all predicted inputs in the history is: {}",
        predicted.iter().sum::<isize>()
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
