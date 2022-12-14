use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fall(pos: [usize; 2], cave: &mut Vec<Vec<bool>>) -> bool {
    if !cave[pos[1] + 1][pos[0]] {
        return fall([pos[0], pos[1] + 1], cave);
    } else if (pos[0] as i32 - 1) < 0 {
        return false;
    } else if !cave[pos[1] + 1][pos[0] - 1] {
        return fall([pos[0] - 1, pos[1]], cave);
    } else if pos[0] + 1 >= cave[pos[1]].len() {
        return false;
    } else if !cave[pos[1] + 1][pos[0] + 1] {
        return fall([pos[0] + 1, pos[1]], cave);
    }

    cave[pos[1]][pos[0]] = true;
    return true;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut rocks = Vec::new();
    let mut min_x = 500;
    let mut max_x = 500;
    let mut max_y = 0;

    for line in reader.lines() {
        let row = line.unwrap();
        let wall = row
            .split(" -> ")
            .map(|x| x.split(",").collect::<Vec<&str>>())
            .map(|x| {
                [
                    x[0].parse::<usize>().unwrap(),
                    x[1].parse::<usize>().unwrap(),
                ]
            })
            .collect::<Vec<[usize; 2]>>();
        max_x = wall.iter().fold(max_x, |acc, x| std::cmp::max(acc, x[0]));
        min_x = wall.iter().fold(min_x, |acc, x| std::cmp::min(acc, x[0]));
        max_y = wall.iter().fold(max_y, |acc, x| std::cmp::max(acc, x[1]));

        rocks.push(wall);
    }
    max_x = 500 + max_y + 3;
    min_x = 500 - max_y - 3;
    max_y += 2;
    rocks.push(vec![[min_x, max_y], [max_x, max_y]]);

    let mut cave = Vec::new();
    for _ in 0..max_y + 1 {
        let mut row = Vec::new();
        for _ in min_x..max_x + 1 {
            row.push(false);
        }
        cave.push(row);
    }

    println!("Size: {}x{}", max_y + 1, max_x - min_x + 1);
    println!("Size: {}x{}", cave.len(), cave[0].len());

    for wall in rocks {
        for i in 1..wall.len() {
            if wall[i - 1][0] == wall[i][0] {
                let from = std::cmp::min(wall[i - 1][1], wall[i][1]);
                let to = std::cmp::max(wall[i - 1][1], wall[i][1]);

                for y in from..to + 1 {
                    cave[y][wall[i - 1][0] - min_x] = true;
                }
            } else {
                let from = std::cmp::min(wall[i - 1][0], wall[i][0]);
                let to = std::cmp::max(wall[i - 1][0], wall[i][0]);

                for x in from..to + 1 {
                    cave[wall[i - 1][1]][x - min_x] = true;
                }
            }
        }
    }

    for i in 0.. {
        if !fall([500 - min_x, 0], &mut cave) {
            println!("{}", i);
            break;
        }
        if cave[0][500 - min_x] {
            println!("{}", i + 1);
            break;
        }
    }

    for row in cave {
        for col in row {
            if col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Ok(())
}
