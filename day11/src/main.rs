use std::error::Error;

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    divisible: i64,
    if_true: usize,
    if_false: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = [
        Monkey {
            items: vec![59, 74, 65, 86],
            operation: |old| old * 19,
            divisible: 7,
            if_true: 6,
            if_false: 2,
        },
        Monkey {
            items: vec![62, 84, 72, 91, 68, 78, 51],
            operation: |old| old + 1,
            divisible: 2,
            if_true: 2,
            if_false: 0,
        },
        Monkey {
            items: vec![78, 84, 96],
            operation: |old| old + 8,
            divisible: 19,
            if_true: 6,
            if_false: 5,
        },
        Monkey {
            items: vec![97, 86],
            operation: |old| old * old,
            divisible: 3,
            if_true: 1,
            if_false: 0,
        },
        Monkey {
            items: vec![50],
            operation: |old| old + 6,
            divisible: 13,
            if_true: 3,
            if_false: 1,
        },
        Monkey {
            items: vec![73, 65, 69, 65, 51],
            operation: |old| old * 17,
            divisible: 11,
            if_true: 4,
            if_false: 7,
        },
        Monkey {
            items: vec![69, 82, 97, 93, 82, 84, 58, 63],
            operation: |old| old + 5,
            divisible: 5,
            if_true: 5,
            if_false: 7,
        },
        Monkey {
            items: vec![81, 78, 82, 76, 79, 80],
            operation: |old| old + 3,
            divisible: 17,
            if_true: 3,
            if_false: 4,
        },
    ];

    let mut monkeys = input.clone();
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let mut items = Vec::new();
            std::mem::swap(&mut items, &mut monkeys[index].items);

            for item in items {
                inspections[index] += 1;
                let item = (monkeys[index].operation)(item) / 3;

                let target = if item % monkeys[index].divisible == 0 {
                    monkeys[index].if_true
                } else {
                    monkeys[index].if_false
                };

                monkeys[target].items.push(item);
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();
    println!("Part 1: {}", inspections[0] * inspections[1]);
    
    let mut monkeys = input.clone();
    let mut inspections = vec![0i64; monkeys.len()];
    let modulo = monkeys.iter().map(|m| m.divisible).reduce(|acc, i| acc * i).unwrap();
    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            let mut items = Vec::new();
            std::mem::swap(&mut items, &mut monkeys[index].items);

            for item in items {
                inspections[index] += 1;
                let item = (monkeys[index].operation)(item) % modulo;

                let target = if item % monkeys[index].divisible == 0 {
                    monkeys[index].if_true
                } else {
                    monkeys[index].if_false
                };

                monkeys[target].items.push(item);
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();
    println!("Part 2: {}", inspections[0] * inspections[1]);

    Ok(())
}
