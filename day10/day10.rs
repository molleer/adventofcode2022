use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut inst = Vec::new();

    for line in reader.lines() {
        let row = line.unwrap();
        let s = row.split(" ").collect::<Vec<&str>>();
        if s[0].eq("noop") {
            inst.push([1, 0]);
        } else {
            inst.push([2, s[1].parse::<i32>().unwrap()]);
        }
    }

    let mut x = 1;
    // Silver
    /*
    let mut sum = 0;
    let mut milestone = 20;
    let mut cycles = 1;

    for i in inst {
        if cycles + i[0] > milestone {
            sum += x * milestone;
            println!("{} {}", milestone, x);
            milestone += 40;
        }
        cycles += i[0];
        x += i[1];
        if cycles > 220 {
            break;
        }
    }

    println!("cycles: {}", cycles);
    println!("sum: {}", sum); */

    let mut wait = 0;
    let mut mem = 0;
    let mut inst_i = 0;
    let mut milestone = 20;
    let mut sum = 0;

    for i in 0..240 {
        if i + 1 > milestone {
            sum += x * milestone;
            milestone += 40;
        }

        wait -= 1;
        if wait <= 0 {
            x += mem;
            wait = inst[inst_i][0];
            mem = inst[inst_i][1];
            inst_i += 1;
        }

        if x == (i % 40) || x - 1 == (i % 40) || x + 1 == (i % 40) {
            print!("#");
        } else {
            print!(".");
        }
        if i % 40 == 0 {
            print!("#\n#");
        }
    }

    println!();
    println!("{}", sum);

    Ok(())
}
