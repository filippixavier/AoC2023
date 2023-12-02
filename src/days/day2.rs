use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

const MAX_CUBES: [usize; 3] = [12, 13, 14];

fn get_input() -> Vec<Vec<[usize; 3]>> {
    let input = fs::read_to_string(Path::new("./input/day2.input"))
        .expect("Something went wrong when opening the input");
    let mut all_games = vec![];
    for line in input.trim().lines() {
        let mut single_game = vec![];
        for game in line.split(&[':', ';']).skip(1) {
            let mut rgb = [0; 3];
            for draw in game.split(',') {
                let cubes = draw.split_whitespace().collect_vec();
                let value = cubes[0].parse::<usize>().unwrap();
                match cubes[1] {
                    "red" => rgb[0] = value,
                    "green" => rgb[1] = value,
                    "blue" => rgb[2] = value,
                    _ => unreachable!(),
                }
            }
            single_game.push(rgb);
        }
        all_games.push(single_game);
    }
    all_games
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let games = get_input();
    let mut valid_games = 0;
    for (id, game) in games.into_iter().enumerate() {
        let max_cubes = game.iter().fold([0; 3], |mut acc, el| {
            acc[0] = acc[0].max(el[0]);
            acc[1] = acc[1].max(el[1]);
            acc[2] = acc[2].max(el[2]);
            acc
        });
        if max_cubes[0] <= MAX_CUBES[0]
            && max_cubes[1] <= MAX_CUBES[1]
            && max_cubes[2] <= MAX_CUBES[2]
        {
            valid_games += id + 1;
        }
    }
    println!("The sum ids of valid games is {}", valid_games);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
