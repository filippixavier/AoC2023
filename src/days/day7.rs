use std::cmp::Ordering;
use std::collections::HashMap;

use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullH,
    FourOAK,
    FiveOAK,
}

fn get_input() -> Vec<(Vec<char>, usize)> {
    let input = fs::read_to_string(Path::new("./input/day7.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap().chars().collect_vec();
            let score = parts.next().unwrap().parse::<usize>().unwrap();
            (hand, score)
        })
        .collect_vec()
}

fn get_card_score(card: &char) -> usize {
    match card {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn get_hand_name(hand: &[char]) -> Hands {
    let mut card_count: HashMap<&char, usize> = HashMap::new();
    for card in hand {
        card_count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut card_count = card_count.values().cloned().collect_vec();
    card_count.sort();
    match card_count.len() {
        1 => Hands::FiveOAK,
        2 => {
            if *card_count.last().unwrap() == 4 {
                Hands::FourOAK
            } else {
                Hands::FullH
            }
        }
        3 => {
            if *card_count.last().unwrap() == 3 {
                Hands::ThreeOAK
            } else {
                Hands::TwoPair
            }
        }
        4 => Hands::OnePair,
        _ => Hands::HighCard,
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut input = get_input();
    input.sort_by(|(left_hand, _), (right_hand, _)| {
        let left_hand_name = get_hand_name(left_hand);
        let right_hand_name = get_hand_name(right_hand);

        if left_hand_name != right_hand_name {
            left_hand_name.cmp(&right_hand_name)
        } else {
            for i in 0..5 {
                if left_hand[i] != right_hand[i] {
                    return get_card_score(&left_hand[i]).cmp(&get_card_score(&right_hand[i]));
                }
            }
            Ordering::Equal
        }
    });
    let mut score = 0;
    for (rank, (_, bet)) in input.into_iter().enumerate() {
        score += (rank + 1) * bet;
    }
    println!("Total winnings are: {:?}", score);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
