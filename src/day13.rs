use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::{fs, io};

#[derive(PartialEq, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn from_str(s: &str) -> Fold {
        let mut iter = s.chars().skip(11);
        let dir = iter.next().unwrap();
        iter.next();
        match dir {
            'x' => Self::X(iter.collect::<String>().parse().unwrap()),
            'y' => Self::Y(iter.collect::<String>().parse().unwrap()),
            c => panic!("unkown fold: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Field<T> {
    content: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl Field<bool> {
    fn new(rows: usize, cols: usize) -> Field<bool> {
        Field {
            content: vec![false; rows * cols],
            rows,
            cols,
        }
    }

    fn from_str(s: &str) -> Field<bool> {
        let points: Vec<_> = s
            .lines()
            .map(|x| {
                let mut iter = x.split(",").map(|x| x.parse::<usize>().unwrap());
                let x = iter.next().unwrap();
                let y = iter.next().unwrap();
                assert!(iter.next().is_none());

                (x, y)
            })
            .collect();

        let cols = points.iter().map(|f| f.0).max().unwrap() + 1;
        let rows = points.iter().map(|f| f.1).max().unwrap() + 1;

        let content = vec![false; cols * rows];

        let mut f = Field {
            content,
            rows,
            cols,
        };

        for p in points {
            f[p] = true;
        }

        f
    }

    fn folded(&self, fold: &Fold) -> Field<bool> {
        match fold {
            Fold::X(col) => self.folded_x(*col),
            Fold::Y(row) => self.folded_y(*row),
        }
    }

    fn folded_y(&self, row: usize) -> Field<bool> {
        let mut flipped = Field::new(row, self.cols);
        for x in 0..self.cols {
            for y in 0..row {
                flipped[(x, y)] = self[(x, y)] || self[(x, 2 * row - y)];
            }
        }

        flipped
    }

    fn folded_x(&self, col: usize) -> Field<bool> {
        let mut flipped = Field::new(self.rows, col);
        for x in 0..col {
            for y in 0..self.rows {
                flipped[(x, y)] = self[(x, y)] || self[(2 * col - x, y)];
            }
        }

        flipped
    }

    fn count(&self) -> usize {
        self.content
            .iter()
            .map(|x| match x {
                true => 1,
                false => 0,
            })
            .sum()
    }
}

impl<T> Field<T> {
    #[allow(dead_code)]
    fn get_coord(&self, index: usize) -> (usize, usize) {
        (index % self.cols, index / self.cols)
    }

    fn get_index(&self, pos: (usize, usize)) -> usize {
        self.cols * pos.1 + pos.0
    }
}

impl Display for Field<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .content
            .chunks(self.cols)
            .map(|x| {
                x.iter()
                    .map(|x| match x {
                        true => '#',
                        false => ' ',
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect();

        write!(f, "{}", s.replace("0", "."))
    }
}

impl<T> Index<(usize, usize)> for Field<T> {
    type Output = T;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.content[self.get_index(pos)]
    }
}

impl<T> IndexMut<(usize, usize)> for Field<T> {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        let index = self.get_index(pos);
        &mut self.content[index]
    }
}

fn parse(filename: &str) -> io::Result<(Field<bool>, Vec<Fold>)> {
    let file = fs::read_to_string(filename)?;
    let mut parts = file.split("\n\n");

    let field = Field::from_str(parts.next().unwrap());
    let folds = parts
        .next()
        .unwrap()
        .lines()
        .map(|x| Fold::from_str(x))
        .collect();

    Ok((field, folds))
}

fn part_one(field: &Field<bool>, folds: &Vec<Fold>) -> usize {
    field.folded(&folds[0]).count()
}

fn part_two(field: &Field<bool>, folds: &Vec<Fold>) {
    let mut iter = folds.iter();
    let mut new = field.folded(iter.next().unwrap());
    while let Some(fold) = iter.next() {
        new = new.folded(fold);
    }
    println!("{}", new);
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let (field, folds) = parse("inputs/day13").unwrap();
    println!("Answer part one: {}", part_one(&field, &folds));
    println!("Answer part two:");
    part_two(&field, &folds);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let (field, folds) = parse("inputs/day13_test").unwrap();
        assert_eq!(folds[0], Fold::Y(7));
        assert_eq!(folds[1], Fold::X(5));
        assert_eq!(field[(6, 10)], true);
    }

    #[test]
    fn test_part_one() {
        let (field, folds) = parse("inputs/day13_test").unwrap();
        println!("{}", field);
        println!("");
        println!("{}", field.folded_y(7));
        assert_eq!(part_one(&field, &folds), 17);
    }
}
