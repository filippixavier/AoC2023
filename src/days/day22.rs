use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct ThreeDimCoordinate {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: ThreeDimCoordinate,
    end: ThreeDimCoordinate,
}

fn get_input() -> Vec<Brick> {
    let input = fs::read_to_string(Path::new("./input/day22.input"))
        .expect("Something went wrong with the input");
    input
        .trim()
        .lines()
        .map(|line| {
            let coordinates: Vec<_> = line.split('~').collect();
            let start = coordinates[0]
                .split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let end = coordinates[1]
                .split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            Brick {
                start: ThreeDimCoordinate {
                    x: start[0],
                    y: start[1],
                    z: start[2],
                },
                end: ThreeDimCoordinate {
                    x: end[0],
                    y: end[1],
                    z: end[2],
                },
            }
        })
        .collect()
}

type Tower = HashMap<usize, (HashSet<usize>, HashSet<usize>)>;

fn get_tower(mut bricks: Vec<Brick>) -> Tower {
    bricks.sort_by(|brick_a, brick_b| brick_a.start.z.cmp(&brick_b.start.z));

    let mut highest = 0;
    let mut bump_map: HashMap<usize, HashMap<(usize, usize), usize>> = HashMap::new();

    let mut tower: HashMap<usize, (HashSet<usize>, HashSet<usize>)> = HashMap::new();

    for (index, brick) in bricks.iter().enumerate() {
        let height = brick.end.z - brick.start.z;
        let mut start_z = 0;

        tower.insert(index, (HashSet::new(), HashSet::new()));

        for z_check in (0..=highest).rev() {
            for x in brick.start.x..=brick.end.x {
                for y in brick.start.y..=brick.end.y {
                    if let Some(brick_id) = bump_map.entry(z_check).or_default().get(&(x, y)) {
                        start_z = z_check + 1;
                        tower.entry(index).and_modify(|(supported_by, _)| {
                            supported_by.insert(*brick_id);
                        });
                        tower.entry(*brick_id).and_modify(|(_, support)| {
                            support.insert(index);
                        });
                    }
                }
            }
            if start_z != 0 {
                break;
            }
        }

        highest = highest.max(start_z + height);

        for z in start_z..=(start_z + height) {
            for x in brick.start.x..=brick.end.x {
                for y in brick.start.y..=brick.end.y {
                    let level_map = bump_map.entry(z).or_default();
                    level_map.insert((x, y), index);
                }
            }
        }
    }
    tower
}

fn get_unremovable_bricks(tower: &Tower) -> HashSet<usize> {
    tower.values().fold(
        HashSet::<usize>::new(),
        |mut acc: HashSet<_>, (supported_by, _)| {
            if supported_by.len() == 1 {
                acc.extend(supported_by);
            }
            acc
        },
    )
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let tower = get_tower(get_input());

    let unremovable = get_unremovable_bricks(&tower);

    println!(
        "🧱💥 We can safely vaporize {} bricks",
        tower.len() - unremovable.len()
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let tower = get_tower(get_input());
    let unremovable: Vec<usize> = get_unremovable_bricks(&tower).iter().cloned().collect();

    let mut drop_count = 0;

    for element in unremovable.iter().rev() {
        let mut removed = HashSet::from([*element]);
        let mut to_check: VecDeque<usize> = tower.get(element).unwrap().1.iter().cloned().collect();

        while let Some(child_id) = to_check.pop_front() {
            let (supported_by, support) = tower.get(&child_id).unwrap();
            if supported_by.is_subset(&removed) {
                removed.insert(child_id);
                to_check.extend(support.iter());
            }
        }
        drop_count += removed.len() - 1;
    }

    println!(
        "🧱💥 By vaporizing the support bricks, we can bring down up to {} bricks",
        drop_count
    );
    Ok(())
}
