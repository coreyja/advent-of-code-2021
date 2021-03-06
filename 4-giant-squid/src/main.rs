use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
struct Cell {
    num: u64,
    marked: bool,
}

#[derive(Debug)]
struct Board<const N: usize> {
    cells: [[Cell; N]; N],
}

impl<const N: usize> Board<N> {
    fn from_str(s: &str) -> Result<Self> {
        fn parse_row<const N: usize>(row: &str) -> Result<[u64; N]> {
            row.split_whitespace()
                .map(|s| s.parse::<u64>().context("parse error"))
                .collect::<Result<Vec<u64>>>()?
                .try_into()
                .map_err(|v: Vec<_>| {
                    anyhow!(
                        "Error making row from vec, vec has length {} where N={}",
                        v.len(),
                        N
                    )
                })
        }

        let cells = s
            .lines()
            .map(|line| {
                let row = parse_row::<N>(line)?;

                Ok(row.map(|num| Cell { num, marked: false }))
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|v: Vec<_>| {
                anyhow!(
                    "Error making array of rows from vec, vec has length {} where N={}",
                    v.len(),
                    N
                )
            })?;

        Ok(Self { cells })
    }

    fn unmarked_cells(&self) -> Vec<((usize, usize), Cell)> {
        (0..N)
            .cartesian_product(0..N)
            .filter(|(i, j)| !self.cells[*i][*j].marked)
            .map(|pos| (pos, self.cells[pos.0][pos.1]))
            .collect()
    }

    fn is_win(&self) -> bool {
        self.is_horizontal_win() || self.is_vertical_win()
    }

    fn rows(&self) -> [[Cell; N]; N] {
        self.cells
    }

    fn columns(&self) -> [[Cell; N]; N] {
        (0..N)
            .map(|i| {
                (0..N)
                    .map(|j| self.cells[j][i])
                    .collect_vec()
                    .try_into()
                    .expect("This is coming from a const N Range, so should always fit")
            })
            .collect_vec()
            .try_into()
            .expect("This is coming from a const N Range, so should always fit")
    }

    fn is_horizontal_win(&self) -> bool {
        self.rows()
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked))
    }

    fn is_vertical_win(&self) -> bool {
        self.columns()
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked))
    }

    fn score(&self) -> Option<u64> {
        if self.is_win() {
            Some(self.unmarked_cells().iter().map(|(_, cell)| cell.num).sum())
        } else {
            None
        }
    }

    fn mark(&mut self, num: u64) {
        for row in self.cells.iter_mut() {
            for cell in row {
                if cell.num == num {
                    cell.marked = true;
                }
            }
        }
    }
}

fn parse_file(s: &str) -> Result<(Vec<u64>, Vec<Board<5>>)> {
    let mut split = s.split("\n\n");
    let chosen_numbers = split
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            s.parse()
                .map_err(|_| anyhow!("Couldn't parse the chosen numbers into u64"))
        })
        .collect::<Result<Vec<_>>>()?;

    let boards = split
        .map(Board::<5>::from_str)
        .collect::<Result<Vec<Board<5>>>>()?;

    Ok((chosen_numbers, boards))
}

fn part1_ans(s: &str) -> Result<Option<u64>> {
    let (chosen_numbers, mut boards) = parse_file(s)?;

    for n in chosen_numbers.iter() {
        for b in boards.iter_mut() {
            b.mark(*n);

            if let Some(score) = b.score() {
                return Ok(Some(score * n));
            }
        }
    }

    Ok(None)
}

fn part2_ans(s: &str) -> Result<Option<u64>> {
    let (chosen_numbers, boards) = parse_file(s)?;

    let mut not_won_boards = boards;

    for n in chosen_numbers.iter() {
        for b in not_won_boards.iter_mut() {
            b.mark(*n);
        }

        if not_won_boards.len() == 1 {
            let last_board = &not_won_boards[0];

            if let Some(score) = last_board.score() {
                return Ok(Some(score * n));
            }
        }

        not_won_boards = not_won_boards.into_iter().filter(|b| !b.is_win()).collect();
    }

    Ok(None)
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Sample: {:?}", part1_ans(include_str!("sample.input"))?);
    println!("My: {:?}", part1_ans(include_str!("my.input"))?);

    println!("Part 2");
    println!("Sample: {:?}", part2_ans(include_str!("sample.input"))?);
    println!("My: {:?}", part2_ans(include_str!("my.input"))?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(Some(4512), part1_ans(include_str!("sample.input")).unwrap());
        assert_eq!(Some(38594), part1_ans(include_str!("my.input")).unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Some(1924), part2_ans(include_str!("sample.input")).unwrap());
        assert_eq!(Some(21184), part2_ans(include_str!("my.input")).unwrap());
    }
}
