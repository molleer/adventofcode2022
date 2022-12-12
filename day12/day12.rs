use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn can_go(from: [usize; 2], to: [i32; 2], map: &Vec<Vec<char>>, visited: &Vec<Vec<usize>>) -> bool {
    if to[0] < 0 || to[1] < 0 {
        return false;
    }
    if to[0] as usize >= map.len() || to[1] as usize >= map[0].len() {
        return false;
    }
    if visited[to[0] as usize][to[1] as usize] != 0 {
        return false;
    }
    let val = map[from[0]][from[1]] as i32;
    if val >= map[to[0] as usize][to[1] as usize] as i32
        || val + 1 == map[to[0] as usize][to[1] as usize] as i32
    {
        return true;
    }
    return false;
}

fn get_actions(
    pos: [usize; 2],
    map: &Vec<Vec<char>>,
    visited: &Vec<Vec<usize>>,
) -> Vec<[usize; 2]> {
    let mut actions: Vec<[usize; 2]> = Vec::new();
    if can_go(pos, [pos[0] as i32 - 1, pos[1] as i32], map, visited) {
        actions.push([pos[0] - 1, pos[1]])
    }
    if can_go(pos, [pos[0] as i32, pos[1] as i32 - 1], map, visited) {
        actions.push([pos[0], pos[1] - 1])
    }
    if can_go(pos, [pos[0] as i32 + 1, pos[1] as i32], map, visited) {
        actions.push([pos[0] + 1, pos[1]])
    }
    if can_go(pos, [pos[0] as i32, pos[1] as i32 + 1], map, visited) {
        actions.push([pos[0], pos[1] + 1])
    }

    return actions;
}

fn shortest_path_len(from: [usize; 2], to: [usize; 2], map: &Vec<Vec<char>>) -> usize {
    let mut visited = Vec::new();
    for i in 0..map.len() {
        visited.push(Vec::new());
        for _ in 0..map[i].len() {
            visited[i].push(0 as usize);
        }
    }
    let mut fringe: Vec<[usize; 2]> = Vec::new();
    fringe.push(from);
    visited[from[0]][from[1]] = 1;

    let mut i = 0;
    while !fringe.contains(&to) && !fringe.is_empty() {
        i += 1;
        let mut new_fringe = Vec::new();
        for p in &fringe {
            visited[p[0]][p[1]] = i;
            new_fringe.append(&mut get_actions(*p, &map, &visited));
        }
        if new_fringe.is_empty() {
            return 100000;
        }
        fringe = new_fringe;
        fringe.sort();
        fringe.dedup();
    }

    return i;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut map = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap().chars().collect::<Vec<char>>();
        map.push(row);
    }

    let mut s_start = [0, 0];
    let mut start = Vec::new();
    let mut goal = [0, 0];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'E' {
                goal = [i, j];
                map[i][j] = 'z';
            } else if map[i][j] == 'a' {
                start.push([i, j]);
            } else if map[i][j] == 'S' {
                s_start = [i, j];
                start.push([i, j]);
                map[i][j] = 'a';
            }
        }
    }

    let mut shortest = 100000;
    for p in start {
        let len = shortest_path_len(p, goal, &map);
        if len < shortest {
            shortest = len;
        }
    }

    println!("Silver: {}", shortest_path_len(s_start, goal, &map));
    println!("Gold: {}", shortest);

    Ok(())
}
