use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use regex::Regex;

type Network = HashMap<String, (String, String)>;

fn get_input() -> (Vec<char>, Network) {
    let input = fs::read_to_string(Path::new("./input/day8.input"))
        .expect("Something went wrong with the input");

    let reg = Regex::new(r"(?<start>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let movement = input.lines().next().unwrap().chars().collect_vec();
    let mut network = HashMap::new();

    for caps in reg.captures_iter(&input) {
        network.insert(
            caps["start"].to_string(),
            (caps["left"].to_string(), caps["right"].to_string()),
        );
    }

    (movement, network)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (movements, network) = get_input();
    let mut steps = 0;
    let mut current_node = "AAA";
    let end_node = "ZZZ";

    for movement in movements.iter().cycle() {
        if current_node == end_node {
            break;
        }
        steps += 1;
        let step = network.get(current_node).unwrap();
        current_node = match *movement {
            'L' => &step.0,
            'R' => &step.1,
            _ => unreachable!(),
        }
    }

    println!("Reaching ZZZ from AAA after {:?} steps", steps);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
