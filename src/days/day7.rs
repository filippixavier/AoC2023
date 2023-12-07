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

fn get_card_score(card: &char, is_joker: bool) -> usize {
    match card {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => {
            if is_joker {
                0
            } else {
                10
            }
        }
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => unreachable!(),
    }
}

fn get_hand_name(hand: &[char], is_joker: bool) -> Hands {
    let mut card_count: HashMap<&char, usize> = HashMap::new();
    for card in hand {
        card_count
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    if is_joker {
        if let Some(joker_count) = card_count.remove(&'J') {
            if card_count.is_empty() {
                return Hands::FiveOAK;
            }
            let (max_card, _) = card_count
                .iter()
                .max_by(|(_, val_a), (_, val_b)| val_a.cmp(val_b))
                .unwrap();
            card_count
                .entry(*max_card)
                .and_modify(|elem| *elem += joker_count);
        }
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
        let left_hand_name = get_hand_name(left_hand, false);
        let right_hand_name = get_hand_name(right_hand, false);

        if left_hand_name != right_hand_name {
            left_hand_name.cmp(&right_hand_name)
        } else {
            for i in 0..5 {
                if left_hand[i] != right_hand[i] {
                    return get_card_score(&left_hand[i], false)
                        .cmp(&get_card_score(&right_hand[i], false));
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
    let mut input = get_input();
    input.sort_by(|(left_hand, _), (right_hand, _)| {
        let left_hand_name = get_hand_name(left_hand, true);
        let right_hand_name = get_hand_name(right_hand, true);

        if left_hand_name != right_hand_name {
            left_hand_name.cmp(&right_hand_name)
        } else {
            for i in 0..5 {
                if left_hand[i] != right_hand[i] {
                    return get_card_score(&left_hand[i], true)
                        .cmp(&get_card_score(&right_hand[i], true));
                }
            }
            Ordering::Equal
        }
    });
    let mut score = 0;
    for (rank, (_, bet)) in input.into_iter().enumerate() {
        score += (rank + 1) * bet;
    }
    println!("Total winnings (jokers included) are: {:?}", score);
    Ok(())
}
