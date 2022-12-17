use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const SHAPES: [[[bool; 4]; 4]; 5] = [
    [
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false, false, false, false],
    ],
    [
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [true, true, false, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
];
const SHAPE_HEIGHTS: [usize; 5] = [1, 3, 3, 4, 2];
const SHAPE_WIDTH: [usize; 5] = [4, 3, 3, 1, 2];
const N_ROCKS: usize = 2022;
const CAVE_HEIGHT: usize = N_ROCKS * 4;

fn fill_shape(x: usize, y: usize, shape: usize, cave: &mut [[bool; 7]; CAVE_HEIGHT]) {
    for i in 0..SHAPE_HEIGHTS[shape] {
        for j in 0..SHAPE_WIDTH[shape] {
            if SHAPES[shape][i][j] {
                cave[i + y][j + x] = SHAPES[shape][i][j];
            }
        }
    }
}

// Return true if there is a collision
fn collision(x: usize, y: usize, shape: usize, cave: &[[bool; 7]; CAVE_HEIGHT]) -> bool {
    for i in 0..4 {
        for j in 0..SHAPE_WIDTH[shape] {
            if SHAPES[shape][i][j] && cave[i + y][j + x] {
                return true;
            }
        }
    }
    return false;
}

// Returns true if the shape can move in the desired direction
fn push(dir: i32, shape: usize, x: i32, y: usize, cave: &[[bool; 7]; CAVE_HEIGHT]) -> bool {
    if x + dir < 0 {
        return false;
    }
    let new_x: usize = (x + dir) as usize;
    if new_x + SHAPE_WIDTH[shape] > 7 {
        return false;
    }
    return !collision(new_x, y, shape, cave);
}

// Return if the shape can fall
fn fall(x: usize, y: usize, shape: usize, cave: &[[bool; 7]; CAVE_HEIGHT]) -> bool {
    if y + SHAPE_HEIGHTS[shape] >= CAVE_HEIGHT {
        return false;
    }
    return !collision(x, y + 1, shape, cave);
}

fn print_cave(cave: &[[bool; 7]; CAVE_HEIGHT]) {
    for y in 0..CAVE_HEIGHT {
        for x in 0..7 {
            if cave[y][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut directions = Vec::new();
    let mut cave = [[false; 7]; CAVE_HEIGHT];

    for line in reader.lines() {
        directions = line
            .unwrap()
            .chars()
            .map(|x| match x {
                '<' => -1,
                '>' => 1,
                _ => 0,
            })
            .collect::<Vec<i32>>();
    }

    let n_dirs = directions.len();
    let mut i = 0;
    let mut smallest_y = CAVE_HEIGHT;
    for rocks in 0..N_ROCKS {
        let shape = rocks % 5;
        let mut y = smallest_y - 4 - SHAPE_HEIGHTS[shape];
        let mut x = 2;
        loop {
            if fall(x as usize, y, shape, &cave) {
                y += 1;
            } else {
                if y < smallest_y {
                    smallest_y = y;
                }
                fill_shape(x as usize, y, shape, &mut cave);
                break;
            }
            if push(directions[i], shape, x, y, &cave) {
                x += directions[i];
            }
            i = (i + 1) % n_dirs;
        }
    }
    // print_cave(&cave);

    println!("{}", CAVE_HEIGHT - smallest_y);

    Ok(())
}
