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
}

#[derive(Debug, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos(x, y)
    }

    fn get_neighbours(&self, board: &Board) -> Vec<Pos> {
        let mut neighbours = Vec::new();
        for x in self.0 - 1..=self.0 + 1 {
            for y in self.1 - 1..=self.1 + 1 {
                if ((x == self.0) ^ (y == self.1))
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

    fn is_low_point(&self, board: &Board) -> bool {
        let neighbours = self.get_neighbours(board);

        neighbours.iter().all(|neighbour| {
            board.cells[self.0 as usize][self.1 as usize]
                < board.cells[neighbour.0 as usize][neighbour.1 as usize]
        })
    }

    fn find_basin_size(&self, board: &Board) -> u32 {
        let mut visited = vec![vec![false; board.cells[0].len()]; board.cells.len()];
        let mut queue = vec![self.clone()];
        let mut size = 0;

        while let Some(pos) = queue.pop() {
            if visited[pos.0 as usize][pos.1 as usize] {
                continue;
            }
            visited[pos.0 as usize][pos.1 as usize] = true;

            let value = board.get(&pos);

            if value != 9 {
                size += 1;
                for n in pos.get_neighbours(board).into_iter() {
                    queue.push(n);
                }
            }
        }

        size
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

fn part2_ans(s: &str) -> Result<u32> {
    let board = parse_input(s)?;
    let size_x = board.cells.len() as i32;
    let size_y = board.cells[0].len() as i32;

    let low = iproduct!((0..size_x), (0..size_y))
        .map(|(x, y)| Pos::new(x, y))
        .filter(|pos| pos.is_low_point(&board))
        .collect_vec();

    let mut basin_sizes = low
        .iter()
        .map(|pos| pos.find_basin_size(&board))
        .collect_vec();

    basin_sizes.sort_unstable();

    Ok(basin_sizes[basin_sizes.len() - 3..].iter().product())
}

fn part1_ans(s: &str) -> Result<i32> {
    let board = parse_input(s)?;

    let size_x = board.cells.len() as i32;
    let size_y = board.cells[0].len() as i32;

    let low = iproduct!((0..size_x), (0..size_y))
        .map(|(x, y)| Pos::new(x, y))
        .filter(|pos| pos.is_low_point(&board))
        .collect_vec();

    Ok(low.iter().map(|pos| board.get(pos)).sum::<i32>() + low.len() as i32)
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
