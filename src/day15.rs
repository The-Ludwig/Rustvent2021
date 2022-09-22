use std::collections::HashSet;
use std::fmt::Display;
use std::{fs, io};

#[derive(Debug, Clone)]
struct Field<T> {
    numbers: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl Field<u8> {
    fn from_str(s: &str) -> Field<u8> {
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

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut u8> {
        if x < self.cols as isize && 0 <= x && y < self.rows as isize && 0 <= y {
            Some(&mut self.numbers[self.cols * y as usize + x as usize])
        } else {
            None
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&u8> {
        if x < self.cols as isize && 0 <= x && y < self.rows as isize && 0 <= y {
            Some(&self.numbers[self.cols * y as usize + x as usize])
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

    fn dijkstra_cost(&self) -> usize {
        let mut cost = Field {
            numbers: vec![usize::MAX; self.cols * self.rows-1],
            rows: self.rows,
            cols: self.cols,
        };
        cost.numbers.insert(0, 0);

        let mut unvisited = HashSet::with_capacity(self.rows * self.cols);
        for x in 0..self.cols {
            for y in 0..self.rows {
                unvisited.insert((x, y));
            }
        }

        unvisited.iter

        loop {
            let current = paths.pop().unwrap();
            let (x, y) = current.1.last().unwrap();

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                match self.get(x + dx, y + dy) {
                    Some(cost) => match current.1.iter().find(|(ox, oy)| ox == x && oy == oy) {
                        None => {
                            let mut newpath = current.1.clone();
                            newpath.push((x + dx, y + dy));
                            paths.push((current.0 + cost, newpath))
                        }
                        Some(_) => (),
                    },
                    None => (),
                }
            }
        }
        0
    }
}

impl Display for Field<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .numbers
            .chunks(self.cols)
            .map(|x| x.iter().map(|x| format!("{}", x)).collect::<String>() + "\n")
            .collect();

        write!(f, "{}", s)
    }
}

fn parse(filename: &str) -> io::Result<Field<u8>> {
    let file = fs::read_to_string(filename)?;

    Ok(Field::from_str(&file))
}

fn part_one(field: &Field<u8>) -> usize {
    0
}

fn part_two(field: &Field<u8>) -> usize {
    0
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
        let field = parse("inputs/day15_test").unwrap();
        println!("{}", field);
        assert_eq!(field.cols, 10);
        assert_eq!(field.rows, 10);
        assert_eq!(field.numbers[2], 6);
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
