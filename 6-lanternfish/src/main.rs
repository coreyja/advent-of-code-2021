use anyhow::Result;

// This represents the number of snakes in each 'lifecycle'
type Swarm = [usize; 9];

fn parse_input(input: &str) -> Result<Swarm> {
    let mut swarm: Swarm = [0; 9];

    for f in input
        .trim()
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|e| anyhow::anyhow!("Couldnt not parse to u32 {}", e))
        })
        .collect::<Result<Vec<u32>>>()?
        .into_iter()
    {
        swarm[f as usize] += 1;
    }

    Ok(swarm)
}

fn simulate_swarm(swarm: &mut Swarm) {
    let zero = swarm[0];

    for i in 0..8 {
        swarm[i] = swarm[i + 1];
    }

    swarm[8] = zero;
    swarm[6] += zero;
}

fn alive_after_days(mut swarm: Swarm, days: usize) -> usize {
    for _ in 0..days {
        simulate_swarm(&mut swarm);
    }

    swarm.iter().sum()
}

fn part1_ans(s: &str) -> Result<usize> {
    let swarm = parse_input(s)?;

    Ok(alive_after_days(swarm, 80))
}

fn part2_ans(s: &str) -> Result<usize> {
    let swarm = parse_input(s)?;

    Ok(alive_after_days(swarm, 256))
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input"))?);
    println!("My: {}", part1_ans(include_str!("my.input"))?);

    println!("Part 2");
    println!("Sample: {}", part2_ans(include_str!("sample.input"))?);
    println!("My: {}", part2_ans(include_str!("my.input"))?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_ans(include_str!("sample.input")).unwrap(), 5934);
        assert_eq!(part1_ans(include_str!("my.input")).unwrap(), 352151);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2_ans(include_str!("sample.input")).unwrap(),
            26984457539
        );
        assert_eq!(part2_ans(include_str!("my.input")).unwrap(), 1601616884019);
    }
}
