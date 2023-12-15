use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<String> {
    let input = fs::read_to_string(Path::new("./input/day15.input"))
        .expect("Something went wrong with the input");
    return input
        .trim()
        .split(',')
        .map(|elem| elem.to_string())
        .collect();
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input_sequence = get_input();
    println!(
        "Total hash value of the input: {}",
        input_sequence.iter().fold(0, |total, step| {
            total
                + step
                    .chars()
                    .fold(0, |acc, elem| ((acc + (elem as usize)) * 17) % 256)
        })
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
