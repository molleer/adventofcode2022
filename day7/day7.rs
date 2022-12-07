use std::collections::{HashMap};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn get_name(cd: &Vec::<String>, stop: usize) -> String {
  let mut name = String::new();
  let mut count = 0;
  for dir in cd {
    if count >= stop {
      break;
    }
    count += 1;
    name.push_str(&"_".to_string());
    name.push_str(&dir.to_string());
  }
  return name;
}
fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut sizes: HashMap<String, i32> = HashMap::new();
    let mut cd = Vec::new();

    for line in reader.lines() {
      let cmd = line.unwrap();
      if cmd.len() > 5 && cmd[..5].eq("$ cd ") {
        if cmd.eq("$ cd ..") {
          cd.pop();
        } else {
          cd.push(cmd[5..].to_string());
        }
        continue;
      }
      
      if !cmd[0..1].eq("$") && !cmd[0..3].eq("dir") {
        let s = cmd.split(" ").collect::<Vec::<&str>>();
        let size = s[0].parse::<i32>().unwrap();
        for i in 0..cd.len() {
          let d = get_name(&cd, i+1);
          let old_val = match sizes.get(&d) {
              Some(x) => *x,
              _ => 0
          };
          sizes.insert(d, old_val + size);
        }
      }
    }

    let mut sum = 0;
    let mut min = 1000000000;
    let required = 30000000 - (70000000 - sizes.get(&"_/".to_string()).unwrap());
    for (_, size) in &sizes {
      if *size <= 100000 {
        sum += size
      }
      if *size < min && *size > required {
        min = *size;
      }
    }

    println!("Silver: {}", sum);
    println!("Min dir: {}", min);
    Ok(())
}