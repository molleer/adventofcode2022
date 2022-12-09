use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// returns the new position of the tail
fn follow(head: [i32; 2], tail: [i32; 2]) -> [i32; 2] {
    // Follow vert hori
    if tail[0] == head[0] {
        if tail[1] + 2 == head[1] {
            return [tail[0], tail[1] + 1];
        } else if tail[1] - 2 == head[1] {
            return [tail[0], tail[1] - 1];
        }
        return tail;
    } else if tail[1] == head[1] {
        if tail[0] + 2 == head[0] {
            return [tail[0] + 1, tail[1]];
        } else if tail[0] - 2 == head[0] {
            return [tail[0] - 1, tail[1]];
        }
        return tail;
    } else if (tail[0] + 1 == head[0] || tail[0] - 1 == head[0])
        && (tail[1] + 1 == head[1] || tail[1] - 1 == head[1])
    {
        return tail;
    }

    // Follow diag
    if tail[1] > head[1] {
        if tail[0] > head[0] {
            return [tail[0] - 1, tail[1] - 1];
        } else {
            return [tail[0] + 1, tail[1] - 1];
        }
    }
    if tail[0] > head[0] {
        return [tail[0] - 1, tail[1] + 1];
    }
    return [tail[0] + 1, tail[1] + 1];
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut dir = Vec::new();
    let mut steps = Vec::new();
    let mut visited = HashSet::new();
    let mut rope: Vec<[i32; 2]> = Vec::new();
    let rope_len = 2; // Silver: 2, Gold: 10
    let mut move_dir = HashMap::new();
    move_dir.insert("R".to_string(), [1, 0]);
    move_dir.insert("L".to_string(), [-1, 0]);
    move_dir.insert("U".to_string(), [0, 1]);
    move_dir.insert("D".to_string(), [0, -1]);

    for line in reader.lines() {
        let row = line.unwrap();
        let s = row.split(" ").collect::<Vec<&str>>();
        dir.push(s[0].to_string());
        steps.push(s[1].parse::<i32>().unwrap());
    }

    for _ in 0..rope_len {
        rope.push([0, 0]);
    }

    visited.insert([0, 0]);
    for i in 0..dir.len() {
        // Move
        let new_dir = move_dir.get(&dir[i]).unwrap();
        for _ in 0..steps[i] {
            rope[0] = [rope[0][0] + new_dir[0], rope[0][1] + new_dir[1]];

            for k in 1..rope_len {
                rope[k] = follow(rope[k - 1], rope[k]);
            }
            visited.insert([rope[rope_len - 1][0], rope[rope_len - 1][1]]);
        }
    }

    println!("Visited: {}", visited.len());

    Ok(())
}
