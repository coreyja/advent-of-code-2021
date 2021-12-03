use itertools::Itertools;

fn into_digit_position_vecs(s: &str) -> Vec<Vec<char>> {
    let mut lines = s.lines().map(|l| l.chars()).collect_vec();
    let mut result = Vec::new();

    let mut chars_left = true;
    while chars_left {
        let mut current_new_line = Vec::new();
        for l in lines.iter_mut() {
            if let Some(c) = l.next() {
                current_new_line.push(c);
            } else {
                chars_left = false;
            }
        }
        if !current_new_line.is_empty() {
            result.push(current_new_line);
        }
    }

    result
}

fn least_and_most_common(v: Vec<char>) -> (char, char) {
    let counts = v.into_iter().counts();

    let counts_v: Vec<_> = counts.into_iter().collect_vec();

    let (min, max) = counts_v
        .iter()
        .minmax_by_key(|x| x.1)
        .into_option()
        .unwrap();

    (min.0, max.0)
}

fn calculate_gamma_and_epsilon(digits: Vec<Vec<char>>) -> (String, String) {
    let mut epsilon_digits = vec![];
    let mut gamma_digits = vec![];

    for digit_line in digits {
        let (epsilon, gamma) = least_and_most_common(digit_line);
        epsilon_digits.push(epsilon);
        gamma_digits.push(gamma);
    }

    (
        epsilon_digits.into_iter().collect(),
        gamma_digits.into_iter().collect(),
    )
}

fn part1_ans(s: &str) -> i64 {
    let digits = into_digit_position_vecs(s);
    let (epsilon, gamma) = calculate_gamma_and_epsilon(digits);

    let epsilon_num = i64::from_str_radix(&epsilon, 2).unwrap();
    let gamma_num = i64::from_str_radix(&gamma, 2).unwrap();

    epsilon_num * gamma_num
}

fn main() {
    println!("Part 1");
    println!("Sample: {:?}", part1_ans(include_str!("sample.input")));
    println!("My: {:?}", part1_ans(include_str!("my.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_digit_position_vecs() {
        let input = "101\n010\n111";

        assert_eq!(
            vec![
                vec!['1', '0', '1'],
                vec!['0', '1', '1'],
                vec!['1', '0', '1']
            ],
            into_digit_position_vecs(input)
        );
    }

    #[test]
    fn test_least_and_most_common() {
        assert_eq!(('1', '0'), least_and_most_common(vec!['0', '1', '0', '0']));
    }
}
