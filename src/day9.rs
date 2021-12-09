use std::collections::HashSet;
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

    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x < self.cols as isize && 0 <= x && y < self.rows as isize && 0 <= y {
            Some(self.numbers[self.cols * y as usize + x as usize])
        } else {
            None
        }
    }

    fn get_low_points(&self) -> Vec<(isize, isize)> {
        let mut low_points = Vec::new();

        for x in 0..self.cols as isize {
            'ns: for y in 0..self.rows as isize {
                let number = self.get(x, y).unwrap();
                for (dx, dy) in [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)] {
                    match self.get(x + dx, y + dy) {
                        Some(n) => {
                            if n <= number {
                                continue 'ns;
                            }
                        }
                        None => (),
                    }
                }
                low_points.push((x, y));
            }
        }
        low_points
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
    field
        .get_low_points()
        .iter()
        .map(|&f| field.get(f.0, f.1).unwrap() as usize + 1)
        .sum()
}

fn part_two(field: &Field) -> usize {
    let mut clusters: Vec<usize> = field
        .get_low_points()
        .iter()
        .map(|&f| {
            let mut visited: HashSet<(isize, isize)> = HashSet::new();
            let mut que: Vec<(isize, isize)> = vec![f];
            let mut size = 0;

            loop {
                let pos = match que.pop() {
                    Some(p) if visited.contains(&p) => continue,
                    Some(p) => p,
                    None => break,
                };

                visited.insert(pos);
                let val = match field.get(pos.0, pos.1) {
                    Some(x) => x,
                    None => continue,
                };

                if val < 9 {
                    size += 1;
                    for (dx, dy) in [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)] {
                        que.push((pos.0 + dx, pos.1 + dy));
                    }
                }
            }
            size
        })
        .collect();

    clusters.sort_unstable();
    let size = clusters.len();
    clusters[size - 1] * clusters[size - 2] * clusters[size - 3]
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let field = parse("inputs/day9").unwrap();

    println!("Answer part one: {}", part_one(&field));
    println!("Answer part two: {}", part_two(&field));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let field = parse("inputs/day9_test").unwrap();
        println!("{}", field);
        assert_eq!(field.cols, 10);
        assert_eq!(field.rows, 5);
        assert_eq!(field.numbers[0], 2);
    }

    #[test]
    fn test_part_one() {
        let field = parse("inputs/day9_test").unwrap();
        println!("{}", field);
        assert_eq!(part_one(&field), 15);
    }

    #[test]
    fn test_part_two() {
        let field = parse("inputs/day9_test").unwrap();
        println!("{}", field);
        assert_eq!(part_two(&field), 1134);
    }
}
