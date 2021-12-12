use std::fmt::Display;
use std::{fs, io};

#[derive(Debug, Clone)]
struct Field {
    numbers: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
}

impl Field {
    fn from_str(s: &str) -> Field {
        let numbers = s
            .replace("\n", "")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let rows = s.lines().count();
        let cols = s.lines().next().unwrap().len();

        Field {
            numbers,
            rows,
            cols,
        }
    }

    fn get(&mut self, x: isize, y: isize) -> Option<&mut u8> {
        if x < self.cols as isize && 0 <= x && y < self.rows as isize && 0 <= y {
            Some(&mut self.numbers[self.cols * y as usize + x as usize])
        } else {
            None
        }
    }

    fn get_coord(&self, index: usize) -> (isize, isize) {
        (
            index as isize % self.cols as isize,
            index as isize / self.cols as isize,
        )
    }

    fn n_step(&mut self, n: usize) -> usize {
        let mut sum = 0;
        for _ in 0..n {
            sum += self.step();
        }
        sum
    }

    fn step(&mut self) -> usize {
        for o in &mut self.numbers {
            *o += 1;
        }

        let mut que: Vec<_> = self
            .numbers
            .iter()
            .enumerate()
            .filter_map(|(idx, &x)| match x == 10 {
                false => None,
                true => Some(self.get_coord(idx)),
            })
            .collect();

        let mut flashes = 0;
        loop {
            let coord = match que.pop() {
                Some(p) => p,
                None => break,
            };

            let val = match self.get(coord.0, coord.1) {
                Some(x) => x,
                None => continue,
            };

            if *val < 10 {
                *val += 1;
            }

            if *val == 10 {
                *val += 1;
                for (dx, dy) in [
                    (0isize, 1isize),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (1, -1),
                    (-1, -1),
                    (-1, 1),
                ] {
                    que.push((coord.0 + dx, coord.1 + dy));
                }
            }
        }

        for o in &mut self.numbers {
            if *o >= 10 {
                *o = 0;
                flashes += 1;
            }
        }

        flashes
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .numbers
            .chunks(self.cols)
            .map(|x| x.iter().map(|x| format!("{}", x)).collect::<String>() + "\n")
            .collect();

        write!(f, "{}", s)
    }
}

fn parse(filename: &str) -> io::Result<Field> {
    let file = fs::read_to_string(filename)?;

    Ok(Field::from_str(&file))
}

fn part_one(field: &Field) -> usize {
    let mut priv_field = field.clone();
    priv_field.n_step(100)
}

fn part_two(field: &Field) -> usize {
    let mut priv_field = field.clone();
    let len = priv_field.numbers.len();
    let mut step = 1;
    while priv_field.step() < len {
        step += 1;
    }
    step
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let field = parse("inputs/day11").unwrap();

    println!("Answer part one: {}", part_one(&field));
    println!("Answer part two: {}", part_two(&field));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let field = parse("inputs/day11_test").unwrap();
        println!("{}", field);
        assert_eq!(field.cols, 10);
        assert_eq!(field.rows, 10);
        assert_eq!(field.numbers[0], 5);
    }

    #[test]
    fn test_part_one() {
        let mut field = parse("inputs/day11_test").unwrap();
        let mut test = field.step();
        println!("{}", field);
        test += field.step();
        println!("{}", field);
        test += field.n_step(18);
        println!("{}", field);
        assert_eq!(field.n_step(80) + test, 1656);
    }

    #[test]
    fn test_part_two() {
        let field = parse("inputs/day11_test").unwrap();
        assert_eq!(part_two(&field), 195);
    }

    #[test]
    fn test_pos() {
        let field = parse("inputs/day11_test").unwrap();
        println!("{}", field);
        assert_eq!(field.get_coord(10), (0, 1));
        assert_eq!(field.get_coord(0), (0, 0));
        assert_eq!(field.get_coord(15), (5, 1));
    }
}
