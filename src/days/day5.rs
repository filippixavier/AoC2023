use std::error::Error;
use std::fs;
use std::path::Path;

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
    unimplemented!("Star 2 not ready");
}
