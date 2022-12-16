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
    let max_x = ranges.iter().map(|r| r.1).max().unwrap() + 1;

    let mut blocked_pos = 0;
    let mut pos = (min_x, row);
    loop {
        let mut blocked = false;
        let mut next_x = max_x;

        for sensor in &sensors {
            let sensor_range = sensor.beacon.0.abs_diff(sensor.position.0) + sensor.beacon.1.abs_diff(sensor.position.1);
            let sensor_distance = pos.0.abs_diff(sensor.position.0) + pos.1.abs_diff(sensor.position.1);
            let vertical_distance = pos.1.abs_diff(sensor.position.1);

            if vertical_distance <= sensor_range {
                if sensor_distance <= sensor_range {
                    blocked = true;
                    next_x = max_x.min(sensor.position.0 + (sensor_range as i32 - vertical_distance as i32) + 1);
                    break;
                } else {
                    next_x = next_x.min(sensor.position.0 - (sensor_range as i32 - vertical_distance as i32));
                }
            }
        }

        if blocked {
            blocked_pos += next_x - pos.0
        }

        pos.0 = next_x;

        if pos.0 == max_x {
            break;
        }
    }

    let mut beacons = sensors.iter().map(|s| s.beacon).collect::<Vec<_>>();
    beacons.sort();
    beacons.dedup();
    blocked_pos -= beacons.iter().filter(|b| b.1 == row).count() as i32;

    println!("Part 1: {blocked_pos}");



    let mut pos = (0i32, 0i32);
    let max = 4000001;
    loop {
        let mut blocked = false;
        let mut next_x = max;

        for sensor in &sensors {
            let sensor_range = sensor.beacon.0.abs_diff(sensor.position.0) + sensor.beacon.1.abs_diff(sensor.position.1);
            let sensor_distance = pos.0.abs_diff(sensor.position.0) + pos.1.abs_diff(sensor.position.1);
            let vertical_distance = pos.1.abs_diff(sensor.position.1);

            if vertical_distance <= sensor_range {
                if sensor_distance <= sensor_range {
                    blocked = true;
                    next_x = max.min(sensor.position.0 + (sensor_range as i32 - vertical_distance as i32) + 1);
                    break;
                } else {
                    next_x = next_x.min(sensor.position.0 - (sensor_range as i32 - vertical_distance as i32));
                }
            }
        }
        
        if !blocked {
            break;
        }

        if next_x < max {
            pos.0 = next_x;
        } else {
            pos = (0, pos.1 + 1);
        }
    }
    
    let frequency = pos.0 as i64 * 4000000 + pos.1 as i64;
    println!("Part 2: {frequency}");

    Ok(())
}
