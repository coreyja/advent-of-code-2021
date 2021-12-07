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

fn median(array: &Vec<u32>) -> f64 {
    if (array.len() % 2) == 0 {
        let ind_left = array.len() / 2 - 1;
        let ind_right = array.len() / 2;
        (array[ind_left] + array[ind_right]) as f64 / 2.0
    } else {
        array[(array.len() / 2)] as f64
    }
}

fn main() -> Result<()> {
    let nums = parse_input(include_str!("sample.input"))?;

    dbg!(median(&nums));

    Ok(())
}
