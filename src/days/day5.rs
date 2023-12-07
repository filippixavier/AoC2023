use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

type ConvertMap = [usize; 3];

fn get_input() -> (Vec<usize>, Vec<Vec<ConvertMap>>) {
    let mut converters = vec![];
    let input = fs::read_to_string(Path::new("./input/day5.input"))
        .expect("Something went wrong with the input");
    let mut parts = input.trim().split("\r\n\r\n");
    let seeds = parts
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    for maps in parts {
        let mut mapper = vec![];
        for line in maps.lines().skip(1) {
            let mut range = [0; 3];
            let values = line
                .split_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            range[0] = values[0];
            range[1] = values[1];
            range[2] = values[2];
            mapper.push(range);
        }
        converters.push(mapper);
    }
    (seeds, converters)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (seeds, maps) = get_input();
    let mut locations = usize::MAX;

    for seed in seeds {
        let mut current = seed;
        for map in &maps {
            for item in map {
                if item[1] <= current && item[1] + item[2] > current {
                    current = item[0] + (current - item[1]);
                    break;
                }
            }
        }
        locations = locations.min(current);
    }

    println!("The lowest seeding location is: {:?}", locations);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (seeds, maps) = get_input();
    let mut seeds_ranges: Vec<[usize; 2]> = seeds
        .chunks(2)
        .map(|seed_range| [seed_range[0], seed_range[0] + seed_range[1]])
        .collect_vec();

    for map in maps.iter() {
        let mut next_range = vec![];
        while !seeds_ranges.is_empty() {
            let mut found = false;
            let [mappable_start, mappable_end] = seeds_ranges.pop().unwrap();
            for &[dest_start, source_start, range] in map {
                let source_end = source_start + range;
                // All ranges excludes outer bounds ([x, y[)
                if source_start < mappable_end && source_end > mappable_start {
                    let (matched_start, matched_end) = (
                        source_start.max(mappable_start),
                        source_end.min(mappable_end),
                    );
                    let unmapped_before = [mappable_start, matched_start];
                    let unmapped_after = [matched_end, mappable_end];

                    if unmapped_after[0] != unmapped_after[1] {
                        seeds_ranges.push(unmapped_after);
                    }
                    if unmapped_before[0] != unmapped_before[1] {
                        seeds_ranges.push(unmapped_before);
                    }
                    let mapped_start = dest_start + matched_start - source_start;
                    let mapped_end = mapped_start + (matched_end - matched_start);

                    next_range.push([mapped_start, mapped_end]);
                    found = true;
                    break;
                }
            }
            if !found {
                next_range.push([mappable_start, mappable_end]);
            }
        }
        seeds_ranges = next_range;
    }

    println!(
        "The lowest seeding location, using seeding ranges, is: {}",
        seeds_ranges.iter().map(|elem| elem[0]).min().unwrap()
    );

    Ok(())
}
