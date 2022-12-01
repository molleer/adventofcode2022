use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let packs: &mut Vec<i32> = &mut Vec::new();
    let mut curr = 0;

    for line in reader.lines() {
      let row = line?;
      if row.eq("") {
          packs.push(curr);
          curr = 0;
          continue;
        }
        let val = row.parse::<i32>().unwrap();
      curr += val;
    }

    let s = packs.len();
    packs.sort();

    println!("{}", packs[s-1] + packs[s-2] + packs[s-3]);
    Ok(())
}