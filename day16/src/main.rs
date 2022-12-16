use std::{error::Error, fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Valve {
    flow_rate: i64,
    tunnels: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day16/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut valves = HashMap::new();
    for line in lines {
        let id = line[6..8].to_owned();
        let mut split = line.split(';');
        let flow_rate = split.next().unwrap()[23..].parse::<i64>().unwrap();
        let tunnels = split.next().unwrap()[23..].split(", ").map(|t| t.trim().to_owned()).collect::<Vec<_>>();
        valves.insert(id, Valve { flow_rate, tunnels });
    }

    let mut distance_from_to = HashMap::new();
    for from in valves.keys() {
        let mut distance_to = HashMap::new();
        for to in valves.keys() {
            distance_to.insert(to.clone(), distance(&valves, from, to));
        }
        distance_from_to.insert(from.clone(), distance_to);
    }

    let mut open = HashSet::new();
    let best = find_best(&valves, &distance_from_to, "AA", &mut open, 30);
    println!("Part 1: {best}");

    Ok(())
}

fn distance(valves: &HashMap<String, Valve>, from: &str, to: &str) -> i64 {
    let mut to_check = HashSet::new();
    to_check.insert(from);

    let mut distance = 0;
    loop {
        for node in std::mem::take(&mut to_check) {
            if node == to {
                return distance;
            }

            for next in &valves[node].tunnels {
                to_check.insert(next);
            }
        }

        distance += 1;
    }
}

fn find_best(valves: &HashMap<String, Valve>, distance_from_to: &HashMap<String, HashMap<String, i64>>, location: &str, open: &mut HashSet<String>, minutes: i64) -> i64 {
    let mut result = 0;

    for (id, valve) in valves {
        if valve.flow_rate > 0 && open.insert(id.to_owned()) {
            let minutes = minutes - distance_from_to[location][id] - 1;
            
            if minutes > 0 {            
                let score = valve.flow_rate * minutes + find_best(valves, distance_from_to, id, open, minutes);
                result = result.max(score);
            }

            open.remove(id);
        }
    }

    result
}