use std::fs::File;
use std::io::{prelude::*, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    println!("{}", part_1()?);
    Ok(())
}

fn part_1() -> Result<i32> {
    let file = File::open("inputs/day_2")?;
    let reader = BufReader::new(file);

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;

    for line in reader.lines() {
        if let Some((command, argument)) = line?.split_once(' ') {
            let argument: i32 = argument.parse()?;
            match command {
                "forward" => horizontal_position += argument,
                "down" => depth += argument,
                "up" => depth -= argument,
                _ => anyhow::bail!("invalid command"),
            }
        }
    }

    Ok(depth * horizontal_position)
}
