use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Hail {
    position: (f32, f32, f32),
    speed: (f32, f32, f32),
}

fn get_input() -> Vec<Hail> {
    let input = fs::read_to_string(Path::new("./input/day24.input"))
        .expect("Something went wrong with the input");

    input
        .trim()
        .lines()
        .map(|line| {
            let splitted_line: Vec<_> = line.split(" @ ").collect();
            let position = splitted_line[0]
                .split(", ")
                .map(str::parse::<f32>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let speed: Vec<_> = splitted_line[1]
                .split(", ")
                .map(|elem| {
                    let x = elem.trim().parse::<isize>().unwrap();
                    x as f32
                })
                .collect();
            Hail {
                position: (position[0], position[1], position[2]),
                speed: (speed[0], speed[1], speed[2]),
            }
        })
        .collect()
}

fn get_intersec_time(hail_a: &Hail, hail_b: &Hail) -> Option<(f32, f32)> {
    let matrix = [
        [hail_a.speed.0, -hail_b.speed.0],
        [hail_a.speed.1, -hail_b.speed.1],
    ];
    let denominator = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    if denominator == 0.0 {
        return None;
    }
    let base = [
        hail_b.position.0 - hail_a.position.0,
        hail_b.position.1 - hail_a.position.1,
    ];
    let denominator = 1.0 / denominator;
    let inverse = [
        [denominator * matrix[1][1], denominator * -matrix[0][1]],
        [denominator * -matrix[1][0], denominator * matrix[0][0]],
    ];
    let coefs = [
        inverse[0][0] * base[0] + inverse[0][1] * base[1],
        inverse[1][0] * base[0] + inverse[1][1] * base[1],
    ];
    Some((coefs[0], coefs[1]))
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let hails = get_input();
    let (min, max) = (200_000_000_000_000.0, 400_000_000_000_000.0);
    let mut intersecting = 0;
    for hails in hails.iter().combinations(2) {
        if let Some((time_a, time_b)) = get_intersec_time(hails[0], hails[1]) {
            if time_a < 0.0 || time_b < 0.0 {
                continue;
            }
            let (pos_x, pos_y) = (
                hails[0].speed.0 * time_a + hails[0].position.0,
                hails[0].speed.1 * time_a + hails[0].position.1,
            );
            if pos_x >= min && pos_x <= max && pos_y >= min && pos_y <= max {
                intersecting += 1;
            }
        }
    }

    println!(
        "There are {} hails colliding within the test area",
        intersecting
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
