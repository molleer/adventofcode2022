use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use regex::Match;

fn to_coord(m: &Match) -> [i32; 2] {
    let re = regex::Regex::new(r"[,=]").unwrap();
    let substr = re.split(m.as_str()).collect::<Vec<&str>>();
    return [substr[1].parse().unwrap(), substr[3].parse().unwrap()];
}

fn man_dist(p1: [i32; 2], p2: [i32; 2]) -> i32 {
    return (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs();
}

fn get_next_x(pos: [i32; 2], sensors: &Vec<[i32; 2]>, distances: &Vec<i32>) -> i32 {
    let mut ans = -1;
    for i in 0..sensors.len() {
        let d = man_dist(pos, sensors[i]);
        if d <= distances[i] {
            let next_x = sensors[i][0] + d - (sensors[i][1] - pos[1]).abs() + 1;
            if next_x > ans {
                ans = next_x;
            }
        }
    }
    return ans;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    let mut distances = Vec::new();
    let max_range = 4000000;

    for line in reader.lines() {
        let row = line.unwrap();
        let re = regex::Regex::new(r"x=(-)?[0-9]+, y=(-)?[0-9]+").unwrap();
        let matches = re.find_iter(&row).collect::<Vec<Match>>();
        let s = to_coord(&matches[0]);
        let b = to_coord(&matches[1]);
        sensors.push(s);
        beacons.push(b);
        distances.push(man_dist(s, b));
    }

    let min_x = std::cmp::min(
        sensors.iter().fold(0, |a, b| std::cmp::min(a, b[0])),
        sensors.iter().fold(0, |a, b| std::cmp::min(a, b[0])),
    );
    let min_y = std::cmp::min(
        sensors.iter().fold(0, |a, b| std::cmp::min(a, b[1])),
        sensors.iter().fold(0, |a, b| std::cmp::min(a, b[1])),
    );
    let max_x = min_x + max_range;
    let max_y = min_y + max_range;

    let mut pos = [min_x, min_y];

    while pos[0] <= max_x && pos[1] <= max_y {
        let next_x = get_next_x(pos, &sensors, &distances);
        if next_x == -1 {
            println!("{}", pos[0] * 4000000 + pos[1]);
            break;
        }
        pos[0] = next_x;
        if pos[0] > max_x {
            pos[0] = min_x;
            pos[1] += 1;
        }
    }

    Ok(())
}
