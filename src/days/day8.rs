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
        let child_node = network.get(current_node).unwrap();
        current_node = match *movement {
            'L' => &child_node.0,
            'R' => &child_node.1,
            _ => unreachable!(),
        }
    }

    println!("Reaching ZZZ from AAA after {:?} steps", steps);

    Ok(())
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(values: &[usize]) -> usize {
    let mut ans = values[0];
    for value in values.iter().skip(1) {
        ans = (ans * value) / gcd(ans.max(*value), ans.min(*value));
    }
    ans
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (movements, network) = get_input();
    let loops_sizes = network
        .keys()
        .filter_map(|start| {
            if start.ends_with('A') {
                let mut node = start;
                for (steps, movement) in movements.iter().cycle().enumerate() {
                    if node.ends_with('Z') {
                        return Some(steps);
                    }
                    let childs = network.get(node).unwrap();
                    node = match movement {
                        'L' => &childs.0,
                        'R' => &childs.1,
                        _ => unreachable!(),
                    }
                }
                None
            } else {
                None
            }
        })
        .collect_vec();
    let steps = lcm(&loops_sizes);
    println!(
        "👻 Reaching all Z-ending nodes at once in {:?} steps 👻",
        steps
    );

    Ok(())
}
