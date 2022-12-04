use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut overlaps = 0;

    for line in reader.lines() {
      let re = Regex::new("[-,]");
      let row = line.unwrap();
      let strs = re.unwrap().split(&row).collect::<Vec<&str>>();
      let nums = strs.iter().map(|&x| x.to_string().parse::<i32>().unwrap()).collect::<Vec::<i32>>();

      // if (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[2] <= nums[0] && nums[3] >= nums[1]) {
      if (nums[0] <= nums[2] && nums[2] <= nums[1]) || (nums[0] <= nums[3] && nums[3] <= nums[1]) || (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[2] <= nums[0] && nums[3] >= nums[1]) {
        overlaps += 1;
      }
    }

    println!("{}", overlaps);
    Ok(())
}