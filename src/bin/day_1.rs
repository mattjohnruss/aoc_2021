use anyhow::Result;

fn main() -> Result<()> {
    part_1()?;

    Ok(())
}

fn part_1() -> Result<()> {
    let depths: Vec<_> = std::fs::read_to_string("inputs/day_1")?
        .split_whitespace()
        .map(|d| d.parse::<usize>().expect("converting string to integer failed"))
        .collect();

    let n_increases = depths
        .windows(2)
        .filter(|d| (d[1] > d[0]))
        .count();

    println!("{}", n_increases);

    Ok(())
}
