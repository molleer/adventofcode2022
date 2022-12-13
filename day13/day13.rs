use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Clone)]
enum Package {
    List(Vec<Package>),
    Numb(i32),
}

fn parse_package(_input: &mut VecDeque<char>) -> Package {
    let mut num = "".to_string();
    let mut list = Vec::new();

    while !_input.is_empty() {
        match _input.pop_front() {
            Some(']') => {
                if !num.is_empty() {
                    list.push(Package::Numb(num.parse::<i32>().unwrap()));
                }
                return Package::List(list);
            }
            Some('[') => list.push(parse_package(_input)),
            Some(',') => {
                if num.is_empty() {
                    continue;
                }
                list.push(Package::Numb(num.parse::<i32>().unwrap()));
                num = "".to_string();
            }
            Some(c) => num.push(c),
            _ => println!("Could not parse char!"),
        }
    }
    println!("Package ended without ']'");
    return Package::Numb(0);
}

fn print_package(pack: &Package) {
    match pack {
        Package::List(x) => {
            print!(" [");
            for i in x {
                print_package(i);
            }
            print!(" ]");
        }
        Package::Numb(n) => print!(" {}", n),
    }
}

fn is_ordered(p1: &Package, p2: &Package) -> Option<bool> {
    /*print!("P1: ");
    print_package(p1);
    print!("\nP2: ");
    print_package(p2);
    println!("\n***********");*/

    match (p1, p2) {
        (Package::List(_), Package::Numb(b)) => {
            return is_ordered(p1, &Package::List(vec![Package::Numb(*b)]));
        }
        (Package::Numb(a), Package::Numb(b)) => {
            if a < b {
                return Some(true);
            } else if a > b {
                return Some(false);
            }
            return None;
        }
        (Package::Numb(a), Package::List(_)) => {
            return is_ordered(&Package::List(vec![Package::Numb(*a)]), p2);
        }
        (Package::List(a), Package::List(b)) => {
            let len = std::cmp::min(a.len(), b.len());
            for i in 0..len {
                match is_ordered(&a[i], &b[i]) {
                    Some(x) => return Some(x),
                    _ => (),
                }
            }
            if a.len() == b.len() {
                return None;
            }
            return Some(a.len() < b.len());
        }
        _ => (),
    }
    return Some(true);
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut packages = Vec::new();

    for line in reader.lines() {
        let mut row = line.unwrap().chars().collect::<VecDeque<char>>();
        if row.is_empty() {
            continue;
        }
        row.pop_front();
        packages.push(parse_package(&mut row));
    }

    for i in (0..packages.len()).step_by(2) {
        match is_ordered(&packages[i], &packages[i + 1]) {
            Some(true) => println!("Ordered!"),
            Some(false) => println!("Not Ordered!"),
            None => println!("Failed to parse!"),
        }
    }

    Ok(())
}
