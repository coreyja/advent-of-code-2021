use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn parse(s: &str) -> Pos {
        let mut iter = s.split(',');
        Pos {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Fold {
    HorizontalAxis(u16),
    VerticalAxis(u16),
}

impl Fold {
    fn parse(s: &str) -> Fold {
        let s = s.strip_prefix("fold along ").unwrap();
        let mut iter = s.split('=');
        let x_or_y = iter.next().unwrap();
        let pos = iter.next().unwrap().parse().unwrap();

        match x_or_y {
            "y" => Fold::HorizontalAxis(pos),
            "x" => Fold::VerticalAxis(pos),
            _ => panic!("invalid fold"),
        }
    }
}

#[derive(Debug)]
struct Input {
    holes: Vec<Pos>,
    folds: VecDeque<Fold>,
}

impl Input {
    fn parse(s: &str) -> Self {
        let mut halves = s.split("\n\n");
        let holes = halves.next().unwrap().lines().map(Pos::parse).collect();

        let folds = halves.next().unwrap().lines().map(Fold::parse).collect();

        Input { holes, folds }
    }

    fn fold_first(&mut self) {
        if let Some(fold) = self.folds.pop_front() {
            match fold {
                Fold::HorizontalAxis(pos) => {
                    self.fold_horizontal(pos);
                }
                Fold::VerticalAxis(pos) => {
                    self.fold_vertical(pos);
                }
            }
        }
    }

    fn fold_horizontal(&mut self, pos: u16) {
        for hole in self.holes.iter_mut() {
            assert_ne!(hole.y, pos as u32);

            if hole.y > pos as u32 {
                let diff = hole.y - pos as u32;
                hole.y = pos as u32 - diff;
            }
        }

        for hole in self.holes.iter() {
            assert!(hole.y < pos.into());
        }

        self.holes.sort();
        self.holes.dedup();
    }

    fn fold_vertical(&mut self, pos: u16) {
        for hole in self.holes.iter_mut() {
            assert_ne!(hole.x, pos as u32);

            if hole.x > pos as u32 {
                let diff = hole.x - pos as u32;

                hole.x = pos as u32 - diff;
            }
        }

        for hole in self.holes.iter() {
            assert!(hole.x < pos.into());
        }

        self.holes.sort();
        self.holes.dedup();
    }
}

fn part1_ans(s: &str) -> usize {
    let mut input = Input::parse(s);

    input.fold_first();

    input.holes.len()
}

fn part2_ans(s: &str) -> String {
    let mut input = Input::parse(s);

    while !input.folds.is_empty() {
        input.fold_first();
    }

    let max_x = input.holes.iter().map(|p| p.x).max().unwrap();
    let max_y = input.holes.iter().map(|p| p.y).max().unwrap();

    let mut lines_grouped: HashMap<u32, Vec<Pos>> = HashMap::new();
    for h in input.holes.into_iter() {
        lines_grouped.entry(h.y).or_default().push(h);
    }

    (0..=max_y)
        .map(|y| {
            let v = lines_grouped.entry(y).or_default();
            (0..=max_x + 1)
                .map(|x| if v.iter().any(|p| p.x == x) { '#' } else { ' ' })
                .collect::<String>()
        })
        .join("\n")
}

fn main() {
    println!("Part 1");
    println!("Sample: {}", part1_ans(include_str!("sample.input")));
    println!("My: {}", part1_ans(include_str!("my.input")));

    println!("Part 2");
    println!("Sample:\n{}", part2_ans(include_str!("sample.input")));
    println!("My:\n{}", part2_ans(include_str!("my.input")));
}
