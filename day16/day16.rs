use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
use std::time::Instant;

use regex::Regex;

fn opt(
    i: usize,
    min: usize,
    visited: usize,
    rates: &Vec<usize>,
    next_index: &Vec<Vec<usize>>,
    mem: &mut HashMap<[usize; 3], usize>,
) -> usize {
    if min == 0 {
        return 0;
    }
    if min == 1 && rates[i] == 0 {
        return 0;
    }
    if mem.contains_key(&[i, min, visited]) {
        return mem[&[i, min, visited]];
    }

    let mut max = 0;
    for next in &next_index[i] {
        let pressure = opt(*next, min - 1, visited, rates, next_index, mem);
        if pressure > max {
            max = pressure;
        }
    }

    if (visited >> i) % 2 == 0 {
        let pressure = rates[i] * min + opt(i, min - 1, visited + 1 << i, rates, next_index, mem);
        max = std::cmp::max(max, pressure);
    }

    mem.insert([i, min, visited], max);
    return max;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut valves = Vec::new();
    let mut rates = Vec::new();
    let mut next_valves = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        let s = row[6..8].to_string();
        valves.push(s);

        let rate = row.split(";").collect::<Vec<&str>>()[0][23..]
            .parse::<usize>()
            .unwrap();
        rates.push(rate);

        let re = Regex::from_str(r"valve(s)? ").unwrap();
        let next = re.split(&row).collect::<Vec<&str>>()[1]
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        next_valves.push(next);
    }

    let start = valves.iter().position(|x| x == "AA").unwrap();
    let mut next_index = Vec::new();

    for vs in next_valves {
        let mut indexes = Vec::new();
        for v in vs {
            indexes.push(valves.iter().position(|x| *x == v).unwrap());
        }
        next_index.push(indexes);
    }

    println!(
        "{}",
        opt(start, 30, 0, &rates, &next_index, &mut HashMap::new())
    );

    Ok(())
}
