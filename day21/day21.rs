use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn eval(
    name: String,
    expressions: &mut HashMap<String, String>,
    numbers: &mut HashMap<String, i128>,
) -> Result<i128, ()> {
    if numbers.contains_key(&name) {
        return Ok(*numbers.get(&name).unwrap());
    }
    if !&expressions.contains_key(&name) {
        return Err(());
    }

    let exp = &expressions
        .get(&name)
        .unwrap()
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let a = match eval(exp[0].to_string(), expressions, numbers) {
        Ok(n) => n,
        _ => return Err(()),
    };
    let b = match eval(exp[2].to_string(), expressions, numbers) {
        Ok(n) => n,
        _ => return Err(()),
    };

    let n = match exp[1].as_str() {
        "-" => a - b,
        "+" => a + b,
        "*" => a * b,
        "/" => a / b,
        _ => 0,
    };
    let _ = &expressions.remove(&name);
    let _ = &numbers.insert(name.to_string(), n);

    return Ok(n);
}

fn eval2(
    name: String,
    expressions: &mut HashMap<String, String>,
    str_numbers: &mut HashMap<String, String>,
    numbers: &mut HashMap<String, i128>,
) -> String {
    if str_numbers.contains_key(&name) {
        return str_numbers.get(&name).unwrap().to_string();
    }

    let exp = &expressions
        .get(&name)
        .unwrap()
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let a = match eval(exp[0].to_string(), expressions, numbers) {
        Ok(n) => n.to_string(),
        _ => eval2(exp[0].to_string(), expressions, str_numbers, numbers),
    };
    let b = match eval(exp[2].to_string(), expressions, numbers) {
        Ok(n) => n.to_string(),
        _ => eval2(exp[2].to_string(), expressions, str_numbers, numbers),
    };

    let new_exp = format!("({}{}{})", a, exp[1], b);
    let _ = &expressions.remove(&name);
    let _ = &str_numbers.insert(name.to_string(), new_exp.to_string());

    return new_exp;
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut expressions = HashMap::new();
    let mut numbers = HashMap::new();
    let mut str_numbers = HashMap::new();

    for line in reader.lines() {
        let row = line.unwrap();
        let exp = row.split(": ").collect::<Vec<&str>>();
        match exp[1].parse::<i128>() {
            Ok(n) => {
                numbers.insert(exp[0].to_string(), n);
                str_numbers.insert(exp[0].to_string(), exp[1].to_string());
            }
            _ => {
                expressions.insert(exp[0].to_string(), exp[1].to_string());
            }
        };
    }

    numbers.remove(&"humn".to_string());
    str_numbers.insert("humn".to_string(), "x".to_string());
    expressions.insert("root".to_string(), "wdzt = dffc".to_string());

    /*println!(
        "root: {}",
        eval("root".to_string(), &mut expressions, &mut numbers)
    );*/

    println!(
        "Insert the following result at https://calculator-online.net/solve-for-x-calculator/"
    );
    println!(
        "{}",
        eval2(
            "root".to_string(),
            &mut expressions,
            &mut str_numbers,
            &mut numbers
        )
    );

    Ok(())
}
