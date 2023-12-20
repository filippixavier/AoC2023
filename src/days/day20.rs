use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

struct Broadcast {
    connected: Vec<String>,
}

impl Broadcast {
    fn new(connected: Vec<String>) -> Self {
        Broadcast { connected }
    }
}

struct FlipFlop {
    status: bool,
    connected: Vec<String>,
}

impl FlipFlop {
    fn new(connected: Vec<String>) -> Self {
        FlipFlop {
            status: false,
            connected,
        }
    }
}

struct Conjuction {
    connecting: HashMap<String, bool>,
    targets: Vec<String>,
}

impl Conjuction {
    fn new(targets: Vec<String>) -> Self {
        Conjuction {
            connecting: HashMap::new(),
            targets,
        }
    }
}

trait Output {
    fn output(&mut self, input: bool, _from: &str) -> Vec<(String, bool)>;
    fn update_connected(&mut self, connected: Vec<String>);
}

impl Output for Broadcast {
    fn output(&mut self, input: bool, _from: &str) -> Vec<(String, bool)> {
        self.connected
            .iter()
            .cloned()
            .map(|elem| (elem, input))
            .collect()
    }
    fn update_connected(&mut self, _connected: Vec<String>) {}
}

impl Output for FlipFlop {
    fn output(&mut self, input: bool, _from: &str) -> Vec<(String, bool)> {
        if !input {
            self.status = !self.status;
            self.connected
                .iter()
                .cloned()
                .map(|key| (key, self.status))
                .collect()
        } else {
            vec![]
        }
    }
    fn update_connected(&mut self, _connected: Vec<String>) {}
}

impl Output for Conjuction {
    fn output(&mut self, input: bool, from: &str) -> Vec<(String, bool)> {
        self.connecting
            .entry(from.to_string())
            .and_modify(|e| *e = input)
            .or_insert(input);
        let sent = !self.connecting.values().all(|signal| *signal);
        self.targets
            .iter()
            .cloned()
            .map(|key| (key, sent))
            .collect()
    }
    fn update_connected(&mut self, connected: Vec<String>) {
        self.connecting = connected.into_iter().map(|key| (key, false)).collect();
    }
}

fn get_input() -> HashMap<String, Box<dyn Output>> {
    let input = fs::read_to_string(Path::new("./input/day20.input"))
        .expect("Something went wrong with the input");
    let mut network: HashMap<String, Box<dyn Output>> = HashMap::new();
    let mut connected: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.trim().lines() {
        let split: Vec<_> = line.split(" -> ").collect();
        let name = split[0].to_string();
        let targets: Vec<_> = split[1].split(", ").map(str::to_string).collect();
        if &name == "broadcaster" {
            network.insert(name, Box::new(Broadcast::new(targets.clone())));
            for target in targets.iter() {
                connected
                    .entry(target.to_string())
                    .and_modify(|elem| elem.push(String::from("broadcaster")))
                    .or_insert(vec![String::from("broadcaster")]);
            }
            continue;
        }
        let mut parser = name.chars();
        let module_type = parser.next().unwrap();
        let name: String = parser.collect();
        for target in targets.iter() {
            connected
                .entry(target.to_string())
                .and_modify(|elem| elem.push(name.clone()))
                .or_insert(vec![name.clone()]);
        }
        match module_type {
            '&' => {
                network.insert(name, Box::new(Conjuction::new(targets)));
            }
            '%' => {
                network.insert(name, Box::new(FlipFlop::new(targets)));
            }
            _ => unreachable!(),
        }
    }
    for (key, value) in connected {
        network
            .entry(key)
            .and_modify(|node| node.update_connected(value));
    }
    network
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut network = get_input();
    let mut low_pulse = 0;
    let mut high_pulse = 0;

    for _ in 0..1000 {
        low_pulse += 1;
        let mut nodes = vec![(String::from("broadcaster"), false, String::from("button"))];
        while !nodes.is_empty() {
            let mut next_round = vec![];
            for (node_name, signal, from) in nodes.iter() {
                if !network.contains_key(node_name) {
                    continue;
                }
                let node = network.get_mut(node_name).unwrap();
                let mut output: Vec<_> = node
                    .output(*signal, from)
                    .into_iter()
                    .map(|(target, signal)| (target, signal, node_name.clone()))
                    .collect();
                if !output.is_empty() {
                    if output[0].1 {
                        high_pulse += output.len();
                    } else {
                        low_pulse += output.len();
                    }
                }
                next_round.append(&mut output);
            }
            nodes = next_round;
        }
    }
    println!(
        "After 1000 button press, the multiplicated total number of pulses is: {}",
        low_pulse * high_pulse
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    unimplemented!("Star 2 not ready");
}
