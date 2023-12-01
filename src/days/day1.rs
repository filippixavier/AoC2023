use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use regex::Regex;

fn get_input() -> Vec<Vec<u32>> {
    let input = fs::read_to_string(Path::new("./input/day1.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect_vec())
        .collect_vec()
}

fn get_input2() -> Vec<Vec<usize>> {
	let input = fs::read_to_string(Path::new("./input/day1.input"))
        .expect("Something went wrong with the input");
    let reg = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let temp = input.trim().lines().map(|line| {
    	let mut values = vec!();
    	// The same match will be added multiple time, but since we only use the first and last match it shouldn't impact the final result
    	for i in 0..line.len() {
    		if let Some(cap) = reg.captures_at(line, i) {
    			let num = match &cap[1] {
    				"one" => 1,
		    		"two" => 2,
		    		"three" => 3,
		    		"four" => 4,
		    		"five" => 5,
		    		"six" => 6,
		    		"seven" => 7,
		    		"eight" => 8,
		    		"nine" => 9,
		    		a => a.parse::<usize>().unwrap()
		    	};
		    	values.push(num);
    		}
    	}
    	values
    }).collect_vec();
    temp
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
    println!(
        "Calibration digit including digit in letters is {:?}",
        get_input2()
            .iter()
            .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
            .sum::<usize>()
    );
    Ok(())
}