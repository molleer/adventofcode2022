use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut map = Vec::new();
    let mut loading_map = true;
    let mut steps = Vec::new();
    let mut turns = Vec::new();
    let width = 151;

    for line in reader.lines() {
        let row = line.unwrap();
        if row.eq("") {
            loading_map = false;
            continue;
        }
        if loading_map {
            let mut line = row.chars().collect::<Vec<char>>();
            line.append(&mut vec![' '; width - line.len()]);
            map.push(line);
        } else {
            let rl_re = Regex::new(r"[RL]").unwrap();
            let num_re = Regex::new(r"[0-9]*").unwrap();
            steps = rl_re
                .split(row.as_str())
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();

            turns = num_re
                .split(row.as_str())
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            turns.pop();
            turns.remove(0);
        }
    }

    let mut x = 0;
    for c in &map[0] {
        if *c == '.' {
            break;
        }
        x += 1;
    }

    let mut dir = [1, 0];
    println!("pos: {:?} dir: {:?}", [x, 0], dir);
    let mut pos = next_pos([x, 0], dir, &map, steps[0]);
    for i in 1..steps.len() {
        dir = match turns[i - 1].as_str() {
            "R" => [-dir[1], dir[0]],
            "L" => [dir[1], -dir[0]],
            _ => {
                println!("Cannot turn: {}", turns[i - 1]);
                dir
            }
        };
        println!("pos: {:?} dir: {:?}", pos, dir);
        pos = next_pos(pos, dir, &map, steps[i]);
    }

    println!(
        "{}",
        1000 * (pos[1] + 1)
            + 4 * (pos[0] + 1)
            + match dir {
                [1, 0] => 0,
                [0, 1] => 1,
                [-1, 0] => 2,
                [0, -1] => 4,
                _ => 0,
            }
    );
    Ok(())
}

fn other_side(pos: [i32; 2], dir: [i32; 2], map: &Vec<Vec<char>>) -> [i32; 2] {
    let mut offset = [0, 0];
    loop {
        offset[0] -= dir[0];
        offset[1] -= dir[1];
        if pos[1] + offset[1] < 0 || pos[1] + offset[1] >= map.len() as i32 {
            break;
        }
        if pos[0] + offset[0] < 0 || pos[0] + offset[0] >= map[0].len() as i32 {
            break;
        }
        if map[(pos[1] + offset[1]) as usize][(pos[0] + offset[0]) as usize] == ' ' {
            break;
        }
    }
    return [pos[0] + offset[0] + dir[0], pos[1] + offset[1] + dir[1]];
}

fn next_pos(pos: [i32; 2], dir: [i32; 2], map: &Vec<Vec<char>>, max_steps: i32) -> [i32; 2] {
    if max_steps <= 0 {
        return pos;
    }
    if pos[1] + dir[1] < 0
        || pos[1] + dir[1] >= map.len() as i32
        || pos[0] + dir[0] < 0
        || pos[0] + dir[0] >= map[0].len() as i32
        || map[(pos[1] + dir[1]) as usize][(pos[0] + dir[0]) as usize] == ' '
    {
        let new_pos = other_side(pos, dir, map);
        println!("new pos: {:?}", new_pos);
        if map[new_pos[1] as usize][new_pos[0] as usize] == '#' {
            return pos;
        }
        return next_pos(new_pos, dir, map, max_steps - 1);
    }
    if map[(pos[1] + dir[1]) as usize][(pos[0] + dir[0]) as usize] == '#' {
        return pos;
    }

    return next_pos([pos[0] + dir[0], pos[1] + dir[1]], dir, map, max_steps - 1);
}
