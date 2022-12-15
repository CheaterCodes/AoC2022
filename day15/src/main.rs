use std::{error::Error, fs::File, io::{BufReader, BufRead}};

#[derive(Clone, Debug)]
struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day15/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut sensors = Vec::new();

    for line in lines {
        let mut split = line.split([',', ':']);
        let sensor_x = split.next().unwrap()[12..].parse::<i32>().unwrap();
        let sensor_y = split.next().unwrap()[3..].parse::<i32>().unwrap();
        let beacon_x = split.next().unwrap()[24..].parse::<i32>().unwrap();
        let beacon_y = split.next().unwrap()[3..].parse::<i32>().unwrap();

        sensors.push(Sensor{
            position: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        })
    }

    let row = 2000000;

    let mut ranges = Vec::new();
    for sensor in &sensors {
        let row_distance = i32::abs_diff(row, sensor.position.1);
        let beacon_distance = i32::abs_diff(sensor.position.0, sensor.beacon.0) + i32::abs_diff(sensor.position.1, sensor.beacon.1);
        let range_len = beacon_distance as i32 - row_distance as i32;
        if range_len >= 0 {
            ranges.push((sensor.position.0 - range_len, sensor.position.0 + range_len));
        }
    }

    let min_x = ranges.iter().map(|r| r.0).min().unwrap();
    let max_x = ranges.iter().map(|r| r.1).max().unwrap();
    let mut blocked_pos = 0;
    for pos in min_x..=max_x {
        if sensors.iter().any(|s| s.beacon == (pos, row)) {
            continue;
        }

        if ranges.iter().any(|range| range.0 <= pos && pos <= range.1) {
            blocked_pos += 1;
        }
    }

    println!("Part 1: {blocked_pos}");

    Ok(())
}
