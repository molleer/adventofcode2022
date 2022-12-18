use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut cubes = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        cubes.push(
            row.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        )
    }

    const MAX_X: usize = 22;
    const MAX_Y: usize = 22;
    const MAX_Z: usize = 22;

    let mut space = [[[false; MAX_Z + 1]; MAX_Y + 1]; MAX_X + 1];
    for c in &cubes {
        space[c[0]][c[1]][c[2]] = true;
    }

    let mut fringe: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];
    while !fringe.is_empty() {
        let mut new_fringe = Vec::new();
        for f in &fringe {
            space[f.0][f.1][f.2] = true;
            if f.0 > 0 && !space[f.0 - 1][f.1][f.2] {
                new_fringe.push((f.0 - 1, f.1, f.2))
            }
            if f.1 > 0 && !space[f.0][f.1 - 1][f.2] {
                new_fringe.push((f.0, f.1 - 1, f.2))
            }
            if f.2 > 0 && !space[f.0][f.1][f.2 - 1] {
                new_fringe.push((f.0, f.1, f.2 - 1))
            }
            if f.0 < MAX_X && !space[f.0 + 1][f.1][f.2] {
                new_fringe.push((f.0 + 1, f.1, f.2))
            }
            if f.1 < MAX_Y && !space[f.0][f.1 + 1][f.2] {
                new_fringe.push((f.0, f.1 + 1, f.2))
            }
            if f.2 < MAX_Z && !space[f.0][f.1][f.2 + 1] {
                new_fringe.push((f.0, f.1, f.2 + 1))
            }
        }
        new_fringe.sort();
        new_fringe.dedup();
        fringe = new_fringe;
    }
    println!("The space has been computed!");

    let mut area = 0;

    for c in &cubes {
        if !cubes.contains(&vec![c[0] + 1, c[1], c[2]])
            && (c[0] + 1 > MAX_X || space[c[0] + 1][c[1]][c[2]])
        {
            area += 1;
        }
        if c[0] <= 0
            || (!cubes.contains(&vec![c[0] - 1, c[1], c[2]]) && space[c[0] - 1][c[1]][c[2]])
        {
            area += 1;
        }
        if !cubes.contains(&vec![c[0], c[1] + 1, c[2]])
            && (c[1] + 1 > MAX_Y || space[c[0]][c[1] + 1][c[2]])
        {
            area += 1;
        }
        if c[1] <= 0
            || (!cubes.contains(&vec![c[0], c[1] - 1, c[2]]) && space[c[0]][c[1] - 1][c[2]])
        {
            area += 1;
        }
        if !cubes.contains(&vec![c[0], c[1], c[2] + 1])
            && (c[2] + 1 > MAX_Z || space[c[0]][c[1]][c[2] + 1])
        {
            area += 1;
        }
        if c[2] <= 0
            || (!cubes.contains(&vec![c[0], c[1], c[2] - 1]) && space[c[0]][c[1]][c[2] - 1])
        {
            area += 1;
        }
    }

    println!("{}", area);

    Ok(())
}
