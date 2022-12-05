use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut box_load = true;
    let n_stacks = 9;
    let mut crates: Vec<VecDeque<char>> = Vec::new();
    let mut instr = Vec::new();

    for _ in 0..n_stacks {
      crates.push(VecDeque::new());
    }

    for line in reader.lines() {
      if box_load {
        let row = line.unwrap().chars().collect::<Vec::<char>>();
        if *row.get(1).unwrap() as i32 == '1' as i32 {
          box_load = false;
          continue;
        }
        for i in 0..n_stacks {
          if *row.get(1+i*4).unwrap() as i32 != ' ' as i32 {
            crates[i].push_front(*row.get(1+i*4).unwrap());
          }
        }
      }  else {
        let row = line.unwrap();
        if row.len() == 0 {
          continue;
        }

        let mut inst = Vec::new();
        let sp = row.split(" ").collect::<Vec<&str>>();
        inst.push(sp[1].parse().unwrap());
        inst.push(sp[3].parse().unwrap());
        inst.push(sp[5].parse().unwrap());
        instr.push(inst);
      }
    }

    // Moving
    for inst in instr {
      // Silver star
      /*for _ in 0..inst[0] {
        let _crate = crates[inst[1] as usize - 1].pop_back().unwrap();
        crates[inst[2] as usize - 1].push_back(_crate);
      }*/
      // Gold star
      let index = crates[inst[1] as usize - 1].len() - inst[0];
      for _ in 0..inst[0] {
        let _crate = crates[inst[1] as usize - 1].remove(index).unwrap();
        crates[inst[2] as usize - 1].push_back(_crate);
      }
    }
    
    for c in crates {
      print!("{}", c[c.len() as usize - 1]);
    }
    
    println!();
    Ok(())
}