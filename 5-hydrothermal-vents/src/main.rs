use std::collections::HashMap;

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

    fn next_towards(&self, other: &Position) -> Result<Position> {
        let mut x = self.x;
        let mut y = self.y;

        if other.x == self.x {
            if self.y < other.y {
                y += 1;
            } else {
                y -= 1;
            }
        } else if other.y == self.y {
            if self.x < other.x {
                x += 1;
            } else {
                x -= 1;
            }
        } else {
            return Err(anyhow!(
                "Can't get next position towards a position that is not adjacent. Self: {:?} Other: {:?}", &self, &other
            ));
        }

        Ok(Position { x, y })
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

    fn to_cells(&self) -> Box<dyn Iterator<Item = Position> + '_> {
        let mut current = self.start;
        let mut done = false;

        let iter = std::iter::from_fn(move || {
            let end = self.end;

            if done {
                return None;
            }

            if current == end {
                done = true;
            }

            let next = current.next_towards(&end).unwrap();

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

fn get_covered_counts(lines: &[Line]) -> HashMap<Position, usize> {
    let mut chained_iter: Box<dyn Iterator<Item = Position> + '_> = Box::new(std::iter::empty());

    for line in lines.iter().filter(|l| l.is_non_diagonal()) {
        chained_iter = Box::new(chained_iter.chain(line.to_cells()));
    }

    chained_iter.counts()
}

fn part1_ans(s: &str) -> Result<usize> {
    let lines = parse_input(s)?;
    let covered_counts = get_covered_counts(&lines);

    Ok(covered_counts
        .iter()
        .filter(|(_key, value)| **value >= 2)
        .count())
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input"))?);
    println!("My: {}", part1_ans(include_str!("my.input"))?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
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
}
