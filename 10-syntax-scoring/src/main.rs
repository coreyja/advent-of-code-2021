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

fn fist_invalid_character(line: &str) -> Option<char> {
    let mut stack = vec![];

    for character in line.chars() {
        match character {
            '(' | '{' | '[' | '<' => stack.push(character),
            ')' | '}' | ']' | '>' => {
                if stack.last().map(close_char) == Some(character) {
                    stack.pop();
                } else {
                    return Some(character);
                }
            }
            _ => panic!("invalid character"),
        }
    }

    None
}

fn part1_ans(s: &str) -> u64 {
    let input = s.trim().lines();

    let corrupt = input.filter_map(fist_invalid_character).collect::<Vec<_>>();

    corrupt.iter().map(score_invalid).sum()
}

fn main() {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input")));
    println!("Sample: {}", part1_ans(include_str!("my.input")));
}
