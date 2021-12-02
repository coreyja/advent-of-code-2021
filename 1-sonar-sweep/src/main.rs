use itertools::Itertools;

fn count_increases(lines: Vec<u64>) -> usize {
    lines.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

fn transform_input(file: &str) -> Vec<u64> {
    file.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn main() {
    let sample_input: Vec<u64> = transform_input(include_str!("sample.input"));
    let my_input: Vec<u64> = transform_input(include_str!("my.input"));

    println!("Sample Answer: {}", count_increases(sample_input));
    println!("My Answer: {}", count_increases(my_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input: Vec<u64> = transform_input(include_str!("sample.input"));
        assert_eq!(count_increases(sample_input), 7);
    }
}
