use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> (Vec<usize>, Vec<usize>) {
    let input = fs::read_to_string(Path::new("./input/day6.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (times, distances) = get_input();
    let mut total = 1;
    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];
        let mut record_breakers = 0;
        for preparation in 1..*time {
            let traveled = (time - preparation) * preparation;
            if traveled > distance {
                record_breakers += 1;
            }
        }
        total *= record_breakers;
    }
    println!(
        "The value got by multiplying all record breakers is {}",
        total
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (times, distances) = get_input();
    let real_time = times
        .iter()
        .map(|elem| elem.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let real_dist = distances
        .iter()
        .map(|elem| elem.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut total = 0;

    for preparation in 1..real_time {
        let traveled = (real_time - preparation) * preparation;
        if traveled > real_dist {
            total += 1;
        }
    }

    println!("The amount of record breakers is {}", total);
    Ok(())
}
