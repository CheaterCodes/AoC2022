use std::error::Error;

struct Monkey {
    items: Vec<i32>,
    operation: fn(i32) -> i32,
    test: fn(i32) -> bool,
    if_true: usize,
    if_false: usize,
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut monkeys = [
        Monkey {
            items: vec![59, 74, 65, 86],
            operation: |old| old * 19,
            test: |val| val % 7 == 0,
            if_true: 6,
            if_false: 2,
        },
        Monkey {
            items: vec![62, 84, 72, 91, 68, 78, 51],
            operation: |old| old + 1,
            test: |val| val % 2 == 0,
            if_true: 2,
            if_false: 0,
        },
        Monkey {
            items: vec![78, 84, 96],
            operation: |old| old + 8,
            test: |val| val % 19 == 0,
            if_true: 6,
            if_false: 5,
        },
        Monkey {
            items: vec![97, 86],
            operation: |old| old * old,
            test: |val| val % 3 == 0,
            if_true: 1,
            if_false: 0,
        },
        Monkey {
            items: vec![50],
            operation: |old| old + 6,
            test: |val| val % 13 == 0,
            if_true: 3,
            if_false: 1,
        },
        Monkey {
            items: vec![73, 65, 69, 65, 51],
            operation: |old| old * 17,
            test: |val| val % 11 == 0,
            if_true: 4,
            if_false: 7,
        },
        Monkey {
            items: vec![69, 82, 97, 93, 82, 84, 58, 63],
            operation: |old| old + 5,
            test: |val| val % 5 == 0,
            if_true: 5,
            if_false: 7,
        },
        Monkey {
            items: vec![81, 78, 82, 76, 79, 80],
            operation: |old| old + 3,
            test: |val| val % 17 == 0,
            if_true: 3,
            if_false: 4,
        },
    ];

    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let mut items = Vec::new();
            std::mem::swap(&mut items, &mut monkeys[index].items);

            for item in items {
                inspections[index] += 1;
                let item = (monkeys[index].operation)(item) / 3;

                let target = if (monkeys[index].test)(item) {
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

    Ok(())
}
