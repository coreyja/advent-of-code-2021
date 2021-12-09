use itertools::Itertools;

const MAPPINGS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

const CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

struct ValidArrangements(Vec<String>);

struct Puzzle {
    order: [Option<char>; 7],
    valid_arrangements: ValidArrangements,
    digits_signals: Vec<String>,
}

impl Puzzle {
    fn solve(&mut self) -> u64 {
        let one_pattern = self
            .valid_arrangements
            .0
            .iter()
            .find(|s| s.len() == 2)
            .unwrap();
        let four_pattern = self
            .valid_arrangements
            .0
            .iter()
            .find(|s| s.len() == 4)
            .unwrap();
        let seven_pattern = self
            .valid_arrangements
            .0
            .iter()
            .find(|s| s.len() == 3)
            .unwrap();

        let character_counts = self
            .valid_arrangements
            .0
            .iter()
            .map(|s| s.to_string())
            .collect::<String>()
            .chars()
            .counts();

        // Determine the top segment
        self.order[0] = Some(
            seven_pattern
                .chars()
                .find(|x| !one_pattern.contains(&x.to_string()))
                .unwrap(),
        );

        self.order[4] = Some(*character_counts.iter().find(|(k, v)| **v == 4).unwrap().0);
        self.order[5] = Some(*character_counts.iter().find(|(k, v)| **v == 9).unwrap().0);
        self.order[1] = Some(*character_counts.iter().find(|(k, v)| **v == 6).unwrap().0);

        let a_or_c = character_counts
            .iter()
            .filter(|(k, v)| **v == 8)
            .map(|(k, v)| k.to_string())
            .collect::<String>();

        self.order[2] = Some(a_or_c.chars().find(|x| self.order[0] != Some(*x)).unwrap());

        let d_or_g = character_counts
            .iter()
            .filter(|(k, v)| **v == 7)
            .map(|(k, v)| k.to_string())
            .collect::<String>();

        self.order[3] = Some(
            d_or_g
                .chars()
                .find(|x| four_pattern.contains(&x.to_string()))
                .unwrap(),
        );

        self.order[6] = Some(
            d_or_g
                .chars()
                .find(|x| !four_pattern.contains(&x.to_string()))
                .unwrap(),
        );

        let what_was_supposed_to_display_digits = self
            .digits_signals
            .iter()
            .map(|s| {
                let corrected_sequence = s
                    .chars()
                    .map(|c| {
                        let index = self.order.iter().position(|x| x == &Some(c)).unwrap();
                        CHARS[index].to_string()
                    })
                    .sorted()
                    .collect::<String>();

                format!(
                    "{}",
                    MAPPINGS
                        .iter()
                        .position(|x| x == &corrected_sequence)
                        .unwrap()
                )
                .chars()
                .next()
                .unwrap()
            })
            .collect::<String>();

        what_was_supposed_to_display_digits.parse().unwrap()
    }
}

fn parse_line(s: &str) -> (ValidArrangements, Vec<&str>) {
    let mut halves = s.trim().split('|');
    let left = halves
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.to_owned())
        .collect_vec();
    let right = halves.next().unwrap().trim().split(' ').collect::<Vec<_>>();

    (ValidArrangements(left), right)
}

fn parse_str(s: &str) -> Vec<(ValidArrangements, Vec<&str>)> {
    s.trim().split('\n').map(parse_line).collect()
}

fn part1_ans(s: &str) -> usize {
    let input = parse_str(s);

    input
        .into_iter()
        .map(|(_, right)| {
            right
                .iter()
                .filter(|x| {
                    let length = x.len();

                    length == 2 || length == 3 || length == 4 || length == 7
                })
                .count()
        })
        .sum()
}

fn part2_ans(s: &str) -> u64 {
    let input = parse_str(s);

    input
        .into_iter()
        .map(|(valid_arrangements, right)| {
            let mut puzzle = Puzzle {
                order: [None, None, None, None, None, None, None],
                valid_arrangements,
                digits_signals: right.into_iter().map(|x| x.to_string()).collect(),
            };

            puzzle.solve()
        })
        .sum()
}

fn main() {
    println!("Part 1");
    println!("Sample: {:?}", part1_ans(include_str!("sample.input")));
    println!("My: {:?}", part1_ans(include_str!("my.input")));

    println!("Part 2");
    println!("Sample: {:?}", part2_ans(include_str!("sample.input")));
    println!("My: {:?}", part2_ans(include_str!("my.input")));
}
