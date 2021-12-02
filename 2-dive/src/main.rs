#[derive(Debug, PartialEq)]
struct Sub {
    pos: Pos,
    aim: u32,
}

impl Sub {
    fn new() -> Self {
        Sub {
            pos: (0, 0),
            aim: 0,
        }
    }

    fn from_moves(moves: &[Move]) -> Self {
        let mut sub = Sub::new();
        sub.eval_all(moves);
        sub
    }

    fn eval(&mut self, m: &Move) {
        match m.direction {
            Direction::Up => self.aim -= m.distance,
            Direction::Down => self.aim += m.distance,
            Direction::Forward => {
                self.pos.0 += m.distance;
                self.pos.1 += self.aim * m.distance;
            }
        }
    }

    fn eval_all(&mut self, moves: &[Move]) {
        for m in moves {
            self.eval(m);
        }
    }
}

type Pos = (u32, u32);

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "forward" => Some(Direction::Forward),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    distance: u32,
}

impl Move {
    fn from_str(s: &str) -> Option<Move> {
        let mut parts = s.split_whitespace();
        let direction = parts.next().and_then(Direction::from_str);
        let distance = parts.next().and_then(|s| s.parse().ok());
        if direction.is_none() || distance.is_none() {
            return None;
        }

        Some(Move {
            direction: direction.unwrap(),
            distance: distance.unwrap(),
        })
    }

    fn eval(&self, starting_pos: Pos) -> Pos {
        let (x, y) = starting_pos;

        match self.direction {
            Direction::Forward => (x + self.distance, y),
            Direction::Up => (x, y - self.distance),
            Direction::Down => (x, y + self.distance),
        }
    }
}

fn parse_file(s: &str) -> Vec<Move> {
    s.lines()
        .map(Move::from_str)
        .collect::<Option<Vec<Move>>>()
        .unwrap()
}

fn eval_simple_moves(moves: &[Move]) -> Pos {
    let mut pos = (0, 0);

    for m in moves {
        pos = m.eval(pos);
    }

    pos
}

fn main() {
    let sample = parse_file(include_str!("sample.input"));
    let sample_ans = eval_simple_moves(&sample);

    let input = parse_file(include_str!("my.input"));
    let ans = eval_simple_moves(&input);

    println!("Part 1");
    println!(
        "Sample Input: {:?} => {}",
        sample_ans,
        sample_ans.0 * sample_ans.1
    );
    println!("My Input: {:?} => {}", ans, ans.0 * ans.1);

    println!("Part 2");
    let sample_sub = Sub::from_moves(&sample);
    println!(
        "Sample Input: {:?} => {}",
        sample_sub,
        sample_sub.pos.0 * sample_sub.pos.1
    );
    let input_sub = Sub::from_moves(&input);
    println!(
        "My Input: {:?} => {}",
        input_sub,
        input_sub.pos.0 * input_sub.pos.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Direction::from_str("forward"), Some(Direction::Forward));
        assert_eq!(Direction::from_str("up"), Some(Direction::Up));
        assert_eq!(Direction::from_str("down"), Some(Direction::Down));

        assert_eq!(
            Move::from_str("forward 10"),
            Some(Move {
                direction: Direction::Forward,
                distance: 10,
            })
        );
    }

    #[test]
    fn test_parse_input() {
        let str = "forward 10\nup 1\ndown 4";

        assert_eq!(
            parse_file(str),
            vec![
                Move {
                    direction: Direction::Forward,
                    distance: 10,
                },
                Move {
                    direction: Direction::Up,
                    distance: 1,
                },
                Move {
                    direction: Direction::Down,
                    distance: 4,
                },
            ]
        );
    }
}
