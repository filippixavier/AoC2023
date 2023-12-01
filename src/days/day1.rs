use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> Vec<Vec<u32>> {
    let input = fs::read_to_string(Path::new("./input/day1.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect_vec())
        .collect_vec()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    println!(
        "Calibration digit is {:?}",
        get_input()
            .iter()
            .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
            .sum::<u32>()
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
