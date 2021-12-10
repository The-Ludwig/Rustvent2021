use std::fs;

#[derive(PartialEq, Eq)]
enum Brace {
    Triangle,
    Round,
    Square,
    Curly,
}

enum Delimiter {
    Open(Brace),
    Close(Brace),
}

impl Brace {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '<' | '>' => Ok(Self::Triangle),
            '(' | ')' => Ok(Self::Round),
            '[' | ']' => Ok(Self::Square),
            '{' | '}' => Ok(Self::Curly),
            _ => Err(format!("Unknown Delimiter {}", c)),
        }
    }

    fn get_score(&self) -> usize {
        use Brace::*;
        match self {
            Round => 3,
            Square => 57,
            Curly => 1197,
            Triangle => 25137,
        }
    }

    fn get_score_two(&self) -> usize {
        use Brace::*;
        match self {
            Round => 1,
            Square => 2,
            Curly => 3,
            Triangle => 4,
        }
    }
}

impl Delimiter {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '<' | '(' | '[' | '{' => Ok(Self::Open(Brace::from_char(c)?)),
            '>' | ')' | ']' | '}' => Ok(Self::Close(Brace::from_char(c)?)),
            _ => Err(format!("Unknown Delimiter {}", c)),
        }
    }

    fn get_score(&self) -> usize {
        match self {
            Self::Open(x) => x.get_score(),
            Self::Close(x) => x.get_score(),
        }
    }

    fn get_score_two(&self) -> usize {
        match self {
            Self::Open(x) => x.get_score_two(),
            Self::Close(x) => x.get_score_two(),
        }
    }
}

fn part_one(filename: &str) -> usize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|f| {
            use Delimiter::{Close, Open};
            let mut stack: Vec<Delimiter> = Vec::new();

            for c in f.chars() {
                let d = Delimiter::from_char(c).unwrap();
                match &d {
                    Open(_) => stack.push(d),
                    Close(c) => match stack.pop() {
                        Some(o) => match &o {
                            Open(p) if *p == *c => (),
                            Close(_) => panic!("Close bracket on stack"),
                            Open(_) => return Some(d.get_score()),
                        },
                        None => return None,
                    },
                }
            }
            None
        })
        .sum()
}

fn part_two(filename: &str) -> usize {
    let mut scores: Vec<_> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .filter_map(|f| {
            use Delimiter::{Close, Open};
            let mut stack: Vec<Delimiter> = Vec::new();

            for c in f.chars() {
                let d = Delimiter::from_char(c).unwrap();
                match &d {
                    Open(_) => stack.push(d),
                    Close(c) => match stack.pop() {
                        Some(o) => match &o {
                            Open(p) if *p == *c => (),
                            Close(_) => panic!("Close bracket on stack"),
                            Open(_) => return None,
                        },
                        None => return None,
                    },
                }
            }

            let mut score = 0usize;
            for d in stack.iter().rev() {
                score *= 5;
                score += d.get_score_two();
            }

            Some(score)
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let res = part_one("inputs/day10");
    println!("Answer part one: {}", res);

    let res = part_two("inputs/day10");
    println!("Answer part two: {}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let res = part_one("inputs/day10_test");
        assert_eq!(res, 26397);
    }

    #[test]
    fn test_part_two() {
        let res = part_two("inputs/day10_test");
        assert_eq!(res, 288957);
    }
}
