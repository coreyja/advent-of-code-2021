use anyhow::{anyhow, Result};
use itertools::{iproduct, Itertools};

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<i32>>,
}

impl Board {
    fn get(&self, pos: &Pos) -> i32 {
        self.cells[pos.0 as usize][pos.1 as usize]
    }

    fn set(&mut self, pos: &Pos, value: i32) {
        self.cells[pos.0 as usize][pos.1 as usize] = value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos(i32, i32);

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos(x, y)
    }

    fn get_neighbours(&self, board: &Board) -> Vec<Pos> {
        let mut neighbours = Vec::new();

        for x in self.0 - 1..=self.0 + 1 {
            for y in self.1 - 1..=self.1 + 1 {
                if !((x == self.0) && (y == self.1))
                    && x >= 0
                    && y >= 0
                    && x < board.cells.len() as i32
                    && y < board.cells[x as usize].len() as i32
                {
                    neighbours.push(Pos::new(x, y));
                }
            }
        }
        neighbours
    }
}

impl Board {
    fn all_positions(&self) -> Vec<Pos> {
        iproduct!(0..self.cells.len() as i32, 0..self.cells[0].len() as i32)
            .map(|(x, y)| Pos::new(x, y))
            .collect()
    }

    fn single_cycle(&mut self) -> Result<usize> {
        let mut flashed = vec![];

        for pos in self.all_positions() {
            let pos = Pos::new(pos.0, pos.1);

            let old_value = self.get(&pos);
            let new_value = old_value + 1;
            self.set(&pos, new_value);
        }

        while let Some(pos) = self
            .all_positions()
            .into_iter()
            .find(|p| !flashed.contains(p) && self.get(p) > 9)
        {
            flashed.push(pos.clone());

            for n in pos.get_neighbours(self) {
                let old_value = self.get(&n);
                let new_value = old_value + 1;
                self.set(&n, new_value);
            }
        }

        for pos in self.all_positions() {
            let old_value = self.get(&pos);

            if old_value > 9 {
                self.set(&pos, 0);
            }
        }

        Ok(flashed.len())
    }
}

fn parse_input(s: &str) -> Result<Board> {
    fn parse_line(s: &str) -> Result<Vec<i32>> {
        s.trim()
            .chars()
            .map(|s| s.to_string().parse().map_err(|_| anyhow!("Parse error")))
            .collect()
    }

    let cells = s
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Vec<_>>>>()?;

    Ok(Board { cells })
}

fn part1_ans(s: &str) -> Result<i32> {
    let mut board = parse_input(s)?;

    let mut sum = 0;

    for _ in 0..100 {
        sum += board.single_cycle()?;
    }

    sum.try_into().map_err(|_| anyhow!("Overflow"))
}

fn part2_ans(s: &str) -> Result<i32> {
    let mut board = parse_input(s)?;

    for i in 0.. {
        let flashed = board.single_cycle()?;

        if flashed == 100 {
            return Ok(i + 1);
        }
    }

    Err(anyhow!("Infinite loop"))
}

fn main() -> Result<()> {
    println!("Part 1");
    println!("Small: {}", part1_ans(include_str!("small.input"))?);
    println!("Sample: {}", part1_ans(include_str!("sample.input"))?);
    println!("My: {}", part1_ans(include_str!("my.input"))?);

    println!();
    println!("Part 2");
    println!("Sample: {}", part2_ans(include_str!("sample.input"))?);
    println!("My: {}", part2_ans(include_str!("my.input"))?);

    Ok(())
}
