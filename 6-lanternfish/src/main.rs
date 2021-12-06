use anyhow::Result;

#[derive(Debug, Clone, Copy)]
struct Fish(u32);

fn parse_input(input: &str) -> Result<Vec<Fish>> {
    input
        .trim()
        .split(',')
        .map(|s| {
            s.parse()
                .map(Fish)
                .map_err(|e| anyhow::anyhow!("Couldnt not parse to u32 {}", e))
        })
        .collect::<Result<Vec<Fish>>>()
}

fn simulate(initial_swarm: &[Fish]) -> Vec<Fish> {
    let mut new_swarm = Vec::with_capacity(initial_swarm.len());

    for fish in initial_swarm {
        let mut new_fish = fish.clone();
        if new_fish.0 == 0 {
            new_fish.0 = 6;
            new_swarm.push(Fish(8));
        } else {
            new_fish.0 -= 1;
        }
        new_swarm.push(new_fish);
    }

    new_swarm
}

fn part1_ans(s: &str) -> Result<usize> {
    const NUM_CYCLES: usize = 80;

    let mut swarm = parse_input(s)?;

    for _ in 0..NUM_CYCLES {
        swarm = simulate(&swarm);
    }

    Ok(swarm.len())
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input"))?);
    println!("My: {}", part1_ans(include_str!("my.input"))?);

    // println!("Part 2");
    // println!("Sample: {}", part2_ans(include_str!("sample.input"))?);
    // println!("My: {}", part2_ans(include_str!("my.input"))?);

    Ok(())
}
