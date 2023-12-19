use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use regex::Regex;

struct Workflow {
    rules: Vec<Box<Rule>>,
    default: String,
}

type Rule = dyn Fn(&Parts) -> Option<String>;

impl Workflow {
    fn create_rule(
        category: char,
        operand: bool,
        value: usize,
        result: String,
    ) -> impl Fn(&Parts) -> Option<String> {
        move |x| {
            let val = x.get(category);
            let compare = if operand { val > value } else { val < value };
            if compare {
                Some(result.clone())
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
struct Parts {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Parts {
    fn get(&self, elem: char) -> usize {
        match elem {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }
}

fn get_input() -> (HashMap<String, Workflow>, Vec<Parts>) {
    let reg_rule = Regex::new(r"([xmas])([<>])(\d+):(\w+)|(\w+)").unwrap();
    let reg_part = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
    let input = fs::read_to_string(Path::new("./input/day19.input"))
        .expect("Something went wrong with the input");
    let mut inputs = input.split("\r\n\r\n");
    let rules = inputs
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut key = String::new();
            let mut workflow = Workflow {
                rules: vec![],
                default: String::new(),
            };
            for (index, cap) in reg_rule.captures_iter(line).enumerate() {
                if index == 0 {
                    key = cap[5].to_string();
                    continue;
                }
                if cap.get(1).is_some() {
                    let category = cap[1].chars().next().unwrap();
                    let operand = &cap[2] == ">";
                    let value = cap[3].parse::<usize>().unwrap();
                    let result = cap[4].to_string();
                    let rule = Workflow::create_rule(category, operand, value, result);
                    workflow.rules.push(Box::new(rule));
                } else {
                    workflow.default = cap[5].to_string();
                }
            }
            (key, workflow)
        })
        .collect();
    let parts: Vec<Parts> = reg_part
        .captures_iter(inputs.next().unwrap())
        .map(|cap| Parts {
            x: cap[1].parse().unwrap(),
            m: cap[2].parse().unwrap(),
            a: cap[3].parse().unwrap(),
            s: cap[4].parse().unwrap(),
        })
        .collect();
    (rules, parts)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (workflows, parts) = get_input();
    let mut total_rating_number = 0;

    for part in parts {
        let mut workflow_id = String::from("in");
        loop {
            let workflow = workflows.get(&workflow_id).unwrap();
            for rule in workflow.rules.iter() {
                if let Some(result) = rule(&part) {
                    workflow_id = result;
                    break;
                } else {
                    workflow_id = workflow.default.clone();
                }
            }
            if &workflow_id == "A" {
                total_rating_number += part.x + part.m + part.a + part.s;
                break;
            } else if &workflow_id == "R" {
                break;
            }
        }
    }
    println!("{}", total_rating_number);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
