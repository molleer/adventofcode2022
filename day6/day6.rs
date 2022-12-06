use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut characters = Vec::new();

    for line in reader.lines() {
      characters = line.unwrap().chars().collect::<Vec<char>>();
    }

    let mut set = HashSet::new();
    let unique_char_len = 14; // gold // silver = 4

    for i in 0..characters.len() {
      set.clear();
      for k in i..i+unique_char_len {
        set.insert(characters[k]);
      }
      if set.len() == unique_char_len {
        println!("{}", i+unique_char_len);
        break;
      }
    }

    Ok(())
}