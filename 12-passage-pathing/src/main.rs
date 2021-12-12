use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_connections(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut connections = HashMap::new();

    for line in input.lines() {
        let mut words = line.split('-');
        let from = words.next().unwrap();
        let to = words.next().unwrap();

        connections.entry(from).or_insert_with(Vec::new).push(to);
        connections.entry(to).or_insert_with(Vec::new).push(from);
    }

    connections
}

fn make_paths_part1<'input>(
    connections: &HashMap<&'input str, Vec<&'input str>>,
    to: &str,
    path: Vec<&'input str>,
) -> Vec<Vec<&'input str>> {
    let mut paths = Vec::new();

    let from = *path.last().unwrap();

    if from == to {
        paths.push(path);
    } else {
        for next in connections.get(from).unwrap() {
            if !path.contains(next) || next.chars().next().unwrap().is_ascii_uppercase() {
                let mut new_path = path.clone();
                new_path.push(next);
                paths.append(&mut make_paths_part1(connections, to, new_path));
            }
        }
    }

    paths
}

fn make_paths_part2<'input>(
    connections: &HashMap<&'input str, Vec<&'input str>>,
    to: &str,
    path: Vec<&'input str>,
) -> Vec<Vec<&'input str>> {
    let mut paths = Vec::new();

    let from = *path.last().unwrap();

    if from == to {
        paths.push(path);
    } else {
        for next in connections.get(from).unwrap() {
            if !path.contains(next)
                || next.chars().next().unwrap().is_ascii_uppercase()
                || (*next != "start"
                    && *next != "end"
                    && path
                        .iter()
                        .filter(|&x| x.chars().next().unwrap().is_ascii_lowercase())
                        .counts()
                        .iter()
                        .all(|(_, count)| *count == 1))
            {
                let mut new_path = path.clone();
                new_path.push(next);
                paths.append(&mut make_paths_part2(connections, to, new_path));
            }
        }
    }

    paths
}

fn part1_ans(s: &str) -> Result<usize> {
    let connections = parse_connections(s);
    let paths = make_paths_part1(&connections, "end", vec!["start"]);

    Ok(paths.len())
}

fn part2_ans(s: &str) -> Result<usize> {
    let connections = parse_connections(s);
    let paths = make_paths_part2(&connections, "end", vec!["start"]);

    Ok(paths.len())
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Small: {}", part1_ans(include_str!("small.input"))?);
    println!("Medium: {}", part1_ans(include_str!("medium.input"))?);
    println!("Large: {}", part1_ans(include_str!("large.input"))?);
    println!("My: {}", part1_ans(include_str!("my.input"))?);

    println!();
    println!("Part 2");
    println!("Small: {}", part2_ans(include_str!("small.input"))?);
    println!("Medium: {}", part2_ans(include_str!("medium.input"))?);
    println!("Large: {}", part2_ans(include_str!("large.input"))?);
    println!("My: {}", part2_ans(include_str!("my.input"))?);

    Ok(())
}
