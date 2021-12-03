use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day_3")?;
    let mut width = 0;
    let input: Vec<_> = input
        .split_whitespace()
        .map(|s| {
            if s.len() > width {
                width = s.len();
            }
            u32::from_str_radix(s, 2).expect("error parsing binary string")
        })
        .collect();

    println!("{:?}", part_1(&input, width)?);

    Ok(())
}

fn part_1(input: &[u32], width: usize) -> Result<u32> {
    let n = input.len();
    let mut gamma: u32 = 0;
    for pos in 0..width {
        let mut pos_total = 0;
        for i in input {
            pos_total += (i >> pos) & 1;
            if pos_total > (n as u32 / 2) + 1 {
                gamma |= 1 << pos;
                break;
            }
        }
    }

    let epsilon = 2_u32.pow(width as u32) - 1 - gamma;
    Ok(gamma * epsilon)
}
