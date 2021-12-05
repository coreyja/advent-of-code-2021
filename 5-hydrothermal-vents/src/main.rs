use std::{cmp::Ordering, collections::HashMap};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn from_str(s: &str) -> Result<Self> {
        s.trim()
            .split(',')
            .map(|s| {
                s.parse()
                    .map_err(|_| anyhow!("Couldn't parse u32 from string"))
            })
            .collect::<Result<Vec<u32>>>()
            .and_then(|v| {
                if v.len() == 2 {
                    Ok(Position { x: v[0], y: v[1] })
                } else {
                    Err(anyhow!("Wrong number of numbers in position"))
                }
            })
    }

    fn next_towards(&self, other: &Position) -> Position {
        let mut x = self.x;
        let mut y = self.y;

        match self.y.cmp(&other.y) {
            Ordering::Less => y += 1,
            Ordering::Greater => y -= 1,
            Ordering::Equal => {}
        }

        match self.x.cmp(&other.x) {
            Ordering::Less => x += 1,
            Ordering::Greater => x -= 1,
            Ordering::Equal => {}
        }

        Position { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split("->");

        let start_str = parts.next().unwrap();
        let end_str = parts.next().unwrap();

        let start = Position::from_str(start_str)?;
        let end = Position::from_str(end_str)?;

        Ok(Self { start, end })
    }

    fn to_cells(&self) -> Box<dyn Iterator<Item = Position>> {
        let mut current = self.start;
        let mut done = false;
        let end = self.end;

        let iter = std::iter::from_fn(move || {
            if done {
                return None;
            }

            if current == end {
                done = true;
            }

            let next = current.next_towards(&end);

            let last = current;
            current = next;

            Some(last)
        });

        Box::new(iter)
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_non_diagonal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

fn parse_input(s: &str) -> Result<Vec<Line>> {
    s.lines().map(|line| Line::from_str(line)).collect()
}

fn get_covered_counts<T>(lines: T) -> HashMap<Position, usize>
where
    T: Iterator<Item = Line>,
{
    let mut chained_iter: Box<dyn Iterator<Item = Position> + '_> = Box::new(std::iter::empty());

    for line in lines {
        chained_iter = Box::new(chained_iter.chain(line.to_cells()));
    }

    chained_iter.counts()
}

fn part1_ans(s: &str) -> Result<usize> {
    let lines = parse_input(s)?.into_iter().filter(|l| l.is_non_diagonal());
    let covered_counts = get_covered_counts(lines);

    Ok(covered_counts
        .iter()
        .filter(|(_key, value)| **value >= 2)
        .count())
}

fn part2_ans(s: &str) -> Result<usize> {
    let lines = parse_input(s)?;
    let covered_counts = get_covered_counts(lines.into_iter());

    Ok(covered_counts
        .iter()
        .filter(|(_key, value)| **value >= 2)
        .count())
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input"))?);
    println!("My: {}", part1_ans(include_str!("my.input"))?);

    println!("Part 2");
    println!("Sample: {}", part2_ans(include_str!("sample.input"))?);
    println!("My: {}", part2_ans(include_str!("my.input"))?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_straight() {
        let l = Line {
            start: Position { x: 0, y: 0 },
            end: Position { x: 0, y: 5 },
        };

        assert_eq!(
            l.to_cells().collect::<Vec<_>>(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 0, y: 1 },
                Position { x: 0, y: 2 },
                Position { x: 0, y: 3 },
                Position { x: 0, y: 4 },
                Position { x: 0, y: 5 },
            ]
        );
    }

    #[test]
    fn test_iter_diagnol() {
        let l = Line {
            start: Position { x: 0, y: 0 },
            end: Position { x: 5, y: 5 },
        };

        assert_eq!(
            l.to_cells().collect::<Vec<_>>(),
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 2, y: 2 },
                Position { x: 3, y: 3 },
                Position { x: 4, y: 4 },
                Position { x: 5, y: 5 },
            ]
        );
    }
}
