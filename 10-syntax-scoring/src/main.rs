fn close_char(c: &char) -> char {
    match c {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        '<' => '>',
        _ => panic!("invalid char"),
    }
}
fn score_invalid(c: &char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char"),
    }
}
fn score_valid(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid char"),
    }
}

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack = vec![];

    for character in line.chars() {
        match character {
            '(' | '{' | '[' | '<' => stack.push(character),
            ')' | '}' | ']' | '>' => {
                if stack.last().map(close_char) == Some(character) {
                    stack.pop();
                } else {
                    return Err(character);
                }
            }
            _ => panic!("invalid character"),
        }
    }

    Ok(stack)
}

fn part1_ans(s: &str) -> u64 {
    let input = s.trim().lines();

    let corrupt = input
        .map(parse_line)
        .filter_map(|result| result.err())
        .collect::<Vec<_>>();

    corrupt.iter().map(score_invalid).sum()
}

fn part2_ans(s: &str) -> u64 {
    let input = s.trim().lines();

    let mut scores = input
        .map(parse_line)
        .filter_map(|result| result.ok())
        .map(|mut stack| {
            stack.reverse();
            stack.iter().map(close_char).collect::<Vec<_>>()
        })
        .map(|stack| {
            let mut score = 0;

            for c in stack {
                score *= 5;
                score += score_valid(&c);
            }

            score
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input")));
    println!("My: {}", part1_ans(include_str!("my.input")));

    println!("Part 2");
    println!("Sample: {}", part2_ans(include_str!("sample.input")));
    println!("My: {}", part2_ans(include_str!("my.input")));
}
