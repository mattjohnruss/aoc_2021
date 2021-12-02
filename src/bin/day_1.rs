use anyhow::Result;

fn main() -> Result<()> {
    let depths: Vec<_> = std::fs::read_to_string("inputs/day_1")?
        .split_whitespace()
        .map(|d| d.parse::<usize>().expect("converting string to integer failed"))
        .collect();

    println!("{}", part_1(&depths)?);
    println!("{}", part_2(&depths)?);

    Ok(())
}

fn part_1(depths: &[usize]) -> Result<usize> {
    let n_increases = depths
        .windows(2)
        .filter(|d| (d[1] > d[0]))
        .count();

    Ok(n_increases)
}

fn part_2(depths: &[usize]) -> Result<usize> {
    let sums_3: Vec<_> = depths
        .windows(3)
        .map(|d| d[0] + d[1] + d[2])
        .collect();

    let n_increases = sums_3
        .windows(2)
        .filter(|d| (d[1] > d[0]))
        .count();

    Ok(n_increases)
}
