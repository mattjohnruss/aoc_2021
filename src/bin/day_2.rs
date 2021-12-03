use anyhow::{Result, bail};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day_2")?;
    let commands: Vec<_> = input
        .lines()
        .map(|line| line.split_once(' ').expect("error parsing line"))
        .map(|(command, argument)| (command, argument.parse::<i32>().expect("error parsing argument")))
        .collect();

    println!("{}", part_1(&commands)?);
    println!("{}", part_2(&commands)?);

    Ok(())
}

fn part_1(commands: &[(&str, i32)]) -> Result<i32> {
    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;

    for (command, argument) in commands {
        match *command {
            "forward" => horizontal_position += argument,
            "down" => depth += argument,
            "up" => depth -= argument,
            _ => bail!("invalid command"),
        }
    }

    Ok(depth * horizontal_position)
}

fn part_2(commands: &[(&str, i32)]) -> Result<i32> {
    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;
    let mut aim = 0;

    for (command, argument) in commands {
        match *command {
            "down" => aim += argument,
            "up" => aim -= argument,
            "forward" => {
                horizontal_position += argument;
                depth += aim * argument;
            }
            _ => bail!("invalid command"),
        }
    }

    Ok(depth * horizontal_position)
}
