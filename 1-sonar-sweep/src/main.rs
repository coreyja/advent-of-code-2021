use itertools::Itertools;

fn count_increases(lines: &[u64]) -> usize {
    lines.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

fn transform_input(file: &str) -> Vec<u64> {
    file.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

fn sum_three_windows(lines: &[u64]) -> Vec<u64> {
    lines
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect()
}

fn main() {
    let sample_input: Vec<u64> = transform_input(include_str!("sample.input"));
    let my_input: Vec<u64> = transform_input(include_str!("my.input"));

    println!("Part 1");
    println!("Sample Answer: {}", count_increases(&sample_input));
    println!("My Answer: {}", count_increases(&my_input));

    println!("Part 2");
    println!(
        "Sample Answer: {}",
        count_increases(&sum_three_windows(&sample_input))
    );
    println!(
        "My Answer: {}",
        count_increases(&sum_three_windows(&my_input))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_a() {
        let sample_input: Vec<u64> = transform_input(include_str!("sample.input"));
        assert_eq!(count_increases(&sample_input), 7);
    }

    #[test]
    fn test_sample_part_b() {
        let sample_input: Vec<u64> = transform_input(include_str!("sample.input"));
        assert_eq!(count_increases(&sum_three_windows(&sample_input)), 5);
    }

    #[test]
    fn test_sum_three() {
        let sample_input: Vec<u64> = transform_input(include_str!("sample.input"));
        assert_eq!(
            sum_three_windows(&sample_input),
            vec![607, 618, 618, 617, 647, 716, 769, 792]
        );
    }
}
