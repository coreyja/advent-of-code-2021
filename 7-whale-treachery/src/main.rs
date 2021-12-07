use anyhow::{anyhow, Result};

fn parse_input(s: &str) -> Result<Vec<u32>> {
    s.trim()
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|e| anyhow!("Couldn't parse input: {}", e))
        })
        .collect()
}

fn diff_u32(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn addition_up_to(n: u32) -> u32 {
    (1..=n).sum()
}

fn sum_of_additions_diff(nums: &[u32], index: u32) -> u32 {
    let mut diff: u32 = 0;

    for n in nums {
        diff += addition_up_to(diff_u32(*n, index));
    }

    diff
}

fn sum_of_diff(nums: &[u32], index: u32) -> u32 {
    let mut diff: u32 = 0;

    for n in nums {
        diff += diff_u32(*n, index);
    }

    diff
}

// fn least_squares_index(nums: &[u32]) -> (usize, u32) {
//     dbg!((0..*nums.iter().max().unwrap())
//         .map(|i| (i, sum_of_diff(nums, i as u32)))
//         .collect_vec());
// }

fn part1_ans(s: &str) -> Result<(usize, u32)> {
    let nums = parse_input(s)?;

    (0..nums.len())
        .map(|i| (i, sum_of_diff(&nums, i as u32)))
        .min_by_key(|(_, x)| *x)
        .ok_or_else(|| anyhow!("No min found"))
}

fn part2_ans(s: &str) -> Result<(usize, u32, u32)> {
    let nums = parse_input(s)?;

    (0..nums.len())
        .map(|i| {
            (
                i,
                sum_of_additions_diff(&nums, i as u32),
                sum_of_diff(&nums, i as u32),
            )
        })
        .min_by_key(|(_, x, _)| *x)
        .ok_or_else(|| anyhow!("No min found"))
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Sample: {:?}", part1_ans(include_str!("sample.input"))?);
    println!("My: {:?}", part1_ans(include_str!("my.input"))?);

    println!("Part 2");
    println!("Sample: {:?}", part2_ans(include_str!("sample.input"))?);
    println!("My: {:?}", part2_ans(include_str!("my.input"))?);

    Ok(())
}
