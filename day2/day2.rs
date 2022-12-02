use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_score(op: char, me: char) -> i32 {
  let op_n = op as i32;
  let me_n = me as i32;
  if op_n == me_n - 23 {
    return me_n - 87 + 3;
  }
  else if me_n == 88 && op_n == 67 {
      return me_n - 87 + 6;
  } else if me_n == 89 && op_n == 65 {
    return me_n - 87 + 6;
  }
  else if me_n == 90 && op_n == 66 {
    return me_n - 87 + 6;
  }
  
  return me_n - 87;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut score:i32 = 0;
    let mut scores = HashMap::new();
    scores.insert("A X".to_string(), 3);
    scores.insert("B X".to_string(), 1);
    scores.insert("C X".to_string(), 2);
    scores.insert("A Y".to_string(), 4);
    scores.insert("B Y".to_string(), 5);
    scores.insert("C Y".to_string(), 6);
    scores.insert("A Z".to_string(), 8);
    scores.insert("B Z".to_string(), 9);
    scores.insert("C Z".to_string(), 7);
    
    
    for line in reader.lines() {
      let s = match line {
        Ok(x) => *scores.get(&x).unwrap(),// get_score(x.chars().collect::<Vec<char>>()[0], x.chars().collect::<Vec<char>>()[2]),
        _ => 0,
      };
      score += s
    }

    println!("{}", score);
    Ok(())
}