use std::collections::HashMap;
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
    let input_sequence = get_input();
    let mut boxes = HashMap::<usize, Vec<(String, usize)>>::new();

    for step in input_sequence {
        let label_chars = step.chars().take_while(|ch| ch.is_ascii_alphabetic());
        let mut iter_step = step.chars().skip_while(|ch| ch.is_ascii_alphabetic());
        let symbol = iter_step.next().unwrap();
        let remaining = iter_step.collect::<String>();

        let box_index = label_chars
            .clone()
            .fold(0, |acc, ch| (((acc + (ch as usize)) * 17) % 256));

        let label: String = label_chars.collect();

        match symbol {
            '-' => {
                if let Some(lenses) = boxes.get_mut(&box_index) {
                    if let Some(position) = lenses
                        .iter()
                        .position(|(lens_label, _)| &label == lens_label)
                    {
                        lenses.remove(position);
                    }
                }
            }
            '=' => {
                let lens_focal = remaining.parse::<usize>().unwrap();
                boxes
                    .entry(box_index)
                    .and_modify(|lenses| {
                        if let Some(position) = lenses.iter().position(|(name, _)| name == &label) {
                            lenses[position].1 = lens_focal;
                        } else {
                            lenses.push((label.clone(), lens_focal))
                        }
                    })
                    .or_insert(vec![(label.clone(), lens_focal)]);
            }
            _ => unreachable!(),
        }
    }

    let focal_power = boxes.iter().fold(0, |x, (box_index, lenses)| {
        x + lenses
            .iter()
            .enumerate()
            .fold(0, |y, (index, (_, focal_strength))| {
                y + (focal_strength * (index + 1) * (box_index + 1))
            })
    });

    println!(
        "After completing the initialisation sequence, the total focal power is: {}",
        focal_power
    );

    Ok(())
}
