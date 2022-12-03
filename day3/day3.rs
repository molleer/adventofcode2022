use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn priority(c: char) -> i32 {
  let val = c as i32;
  if val > 90 { //lower case
    return val - 96;
  }
  return val - 64 + 26;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut prios = 0;
    let mut lines = Vec::new();
    
    for line in reader.lines() {
      lines.push(line.unwrap());
    }

    for i in 0..lines.len()/3 {
      let mut items = HashSet::new();
      let mut items2 = HashSet::new();
      
      for c in lines.get(i*3).unwrap().chars() {
        items.insert(c);
      }

      for c in lines.get(i*3+1).unwrap().chars() {
        items2.insert(c);
      }

      for c in lines.get(i*3+2).unwrap().chars() {
        if items.contains(&c) && items2.contains(&c) {
          let p = priority(c);
          prios += p;
          break;
        }
      }
    }

    println!("{}", prios);

    Ok(())
}