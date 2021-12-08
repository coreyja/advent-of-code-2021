fn parse_line(s: &str) -> (Vec<&str>, Vec<&str>) {
    let mut halves = s.trim().split('|');
    let left = halves.next().unwrap().trim().split(' ').collect::<Vec<_>>();
    let right = halves.next().unwrap().trim().split(' ').collect::<Vec<_>>();

    (left, right)
}

fn parse_str(s: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
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

fn main() {
    println!("Part 1");
    println!("Sample: {:?}", part1_ans(include_str!("sample.input")));
    println!("My: {:?}", part1_ans(include_str!("my.input")));
}
