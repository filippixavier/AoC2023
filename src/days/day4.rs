use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn get_input() -> Vec<(Vec<usize>, Vec<usize>)> {
    let input = fs::read_to_string(Path::new("./input/day4.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let winning = parts
                .next()
                .unwrap()
                .split_whitespace()
                .skip(2)
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let scratched = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            (winning, scratched)
        })
        .collect_vec()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let cards = get_input();
    let mut result = 0;
    for (winning, scratched) in cards {
        let won = scratched
            .iter()
            .filter(|value| winning.contains(value))
            .count();
        if won > 0 {
            result += 2_u32.pow(won as u32 - 1);
        }
    }

    println!("All these cards are worth {:?} points", result);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let cards = get_input();
    let end = cards.len();
    let mut cards_count = vec![1; end];
    for (index, (winning, scratched)) in cards.into_iter().enumerate() {
        let won = scratched
            .iter()
            .filter(|value| winning.contains(value))
            .count();
        if won > 0 && index + 1 < end {
            for i in index + 1..=(index + won).min(end - 1) {
                cards_count[i] += cards_count[index];
            }
        }
    }

    println!(
        "Ending up with a total of {:?} cards",
        cards_count.iter().sum::<u32>()
    );

    Ok(())
}
