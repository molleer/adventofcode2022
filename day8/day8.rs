use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn isVisible(i: usize, j: usize, forrest: &Vec<Vec<i32>>) -> bool {
  let mut visible = true;
  let height = forrest[i][j];
  let len = forrest.len();

  for k in 0..i {
    if forrest[k][j] >= height {
      visible = false;
      break;
    } 
  }

  if visible {
    return true;
  }
  visible = true;

  for k in 0..j {
    if forrest[i][k] >= height {
      visible = false;
      break;
    } 
  }

  if visible {
    return true;
  }
  visible = true;

  for k in (i+1)..len {
    if forrest[k][j] >= height {
      visible = false;
      break;
    } 
  }

  if visible {
    return true;
  }
  visible = true;

  for k in (j+1)..len {
    if forrest[i][k] >= height {
      visible = false;
      break;
    } 
  }

  return visible;
}

fn scenicScore(i: usize, j: usize, forrest: &Vec<Vec<i32>>) -> i32 {
  let mut score = 1;
  let height = forrest[i][j];
  let len = forrest.len();
  let mut count = 0;

  for k in (0..i).rev() {
    count += 1;
    if forrest[k][j] >= height {
      break;
    }
  }
  score *= count;

  count = 0;

  for k in (0..j).rev() {
    count += 1;
    if forrest[i][k] >= height {
      break;
    }
  }
  score *= count;

  count = 0;

  for k in (i+1)..len {
    count += 1;
    if forrest[k][j] >= height {
      break;
    }
  }
  score *= count;

  count = 0;

  for k in (j+1)..len {
    count += 1;
    if forrest[i][k] >= height {
      break;
    }
  }
  score *= count;

  return score;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut forrest = Vec::new();

    for line in reader.lines() {
      forrest.push(line.unwrap().chars().map(|x| x as i32 - 48).collect::<Vec<i32>>())
    }

    let len = forrest.len();
    let mut max = 0;
    for i in 0..len {
      for j in 0..len {
        let score = scenicScore(i, j, &forrest);
        if score > max {
          max = score;
        }
      }
    }

    println!("{}", max);

    Ok(())
}