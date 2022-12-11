use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut i = 0;
    let mut items: Vec<Vec<u128>> = Vec::new();
    let mut operation: Vec<String> = Vec::new();
    let mut op_val = Vec::new();
    let mut test = Vec::new();
    let mut direction: Vec<[usize; 2]> = Vec::new();

    let mut saved_test = 0;

    for line in reader.lines() {
        let row = line.unwrap();
        match i % 7 {
            1 => items.push(
                row.split(": ").collect::<Vec<&str>>()[1]
                    .split(", ")
                    .map(|x| x.parse::<u128>().unwrap())
                    .collect::<Vec<u128>>(),
            ),
            2 => {
                let op = row.split("old ").collect::<Vec<&str>>()[1]
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                if op[1].eq("old") {
                    operation.push("^".to_string());
                    op_val.push(2)
                } else {
                    operation.push(op[0].to_string());
                    op_val.push(op[1].parse::<u128>().unwrap());
                }
            }
            3 => test.push(row.split(" ").last().unwrap().parse::<u128>().unwrap()),
            4 => saved_test = row.split(" ").last().unwrap().parse::<usize>().unwrap(),
            5 => direction.push([
                saved_test,
                row.split(" ").last().unwrap().parse::<usize>().unwrap(),
            ]),
            _ => print!(""),
        }

        i += 1;
    }

    let n_monkeys = (i + 1) / 7;
    let mut inspect_count = Vec::new();
    let test_prod = test.iter().fold(1, |x, y| x * y);

    for i in 0..n_monkeys {
        inspect_count.push(0);
        println!(
            "Monkey {}, test: {}, dir: [{}, {}], op: {}{}",
            i, test[i], direction[i][0], direction[i][1], operation[i], op_val[i]
        )
    }

    for _ in 0..10000 {
        let mut new_items = (0..n_monkeys)
            .map(|_| Vec::new())
            .collect::<Vec<Vec<u128>>>();
        for m in 0..n_monkeys {
            for item in &items[m] {
                inspect_count[m] += 1;
                let level = match operation[m].as_str() {
                    "*" => item * op_val[m],
                    "+" => item + op_val[m],
                    "^" => item * item,
                    _ => *item,
                } % test_prod; // Silver: / 3;

                if (level % test[m]) == 0 {
                    new_items[direction[m][0]].push(level);
                } else {
                    new_items[direction[m][1]].push(level);
                }
            }
            items[m] = Vec::new();
            for m2 in 0..n_monkeys {
                items[m2].append(&mut new_items[m2]);
            }
        }
    }

    inspect_count.sort();
    println!(
        "{} * {}",
        inspect_count[n_monkeys - 2],
        inspect_count[n_monkeys - 1]
    );

    Ok(())
}
