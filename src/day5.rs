use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::{fs, io};

#[derive(Debug, Clone)]
struct Field {
    numbers: Vec<u32>,
    pub rows: usize,
    pub cols: usize,
    min_x: i32,
    min_y: i32,
}

#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    ///  s in the form 'x1,y1 -> x2,y2
    fn from_str(s: &str) -> Line {
        let nums: Vec<Vec<i32>> = s
            .split(" -> ")
            .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
            .collect();

        Line {
            p1: Point {
                x: nums[0][0],
                y: nums[0][1],
            },
            p2: Point {
                x: nums[1][0],
                y: nums[1][1],
            },
        }
    }
}

impl Field {
    fn update_min_max(val: i32, min: &mut i32, max: &mut i32) {
        if val < *min {
            *min = val;
        } else if val > *max {
            *max = val;
        }
    }

    fn from_str(lines: &str, diagonal: bool) -> Field {
        let lines: Vec<_> = lines.lines().map(|l| Line::from_str(l)).collect();
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (0i32, 0i32, 0i32, 0i32);
        for line in &lines {
            Self::update_min_max(line.p1.x, &mut min_x, &mut max_x);
            Self::update_min_max(line.p2.x, &mut min_x, &mut max_x);
            Self::update_min_max(line.p1.y, &mut min_y, &mut max_y);
            Self::update_min_max(line.p2.y, &mut min_y, &mut max_y);
        }

        let numbers = vec![0; ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize];
        let mut field = Field {
            numbers,
            rows: (max_y - min_y) as usize + 1,
            cols: (max_x - min_x) as usize + 1,
            min_x,
            min_y,
        };

        for line in lines {
            field.add_line(&line, diagonal);
        }

        field
    }

    fn get_iter(p1: i32, p2: i32) -> Box<dyn Iterator<Item = i32>> {
        match p2 >= p1 {
            true => Box::new(p1..=p2),
            false => Box::new((p2..=p1).rev()),
        }
    }

    fn add_line(&mut self, line: &Line, diagonal: bool) {
        if line.p1.x == line.p2.x {
            for y in Field::get_iter(line.p1.y, line.p2.y) {
                self[(line.p1.x, y)] += 1;
            }
        } else if line.p1.y == line.p2.y {
            for x in Field::get_iter(line.p1.x, line.p2.x) {
                self[(x, line.p1.y)] += 1;
            }
        } else if diagonal {
            for (x, y) in
                Field::get_iter(line.p1.x, line.p2.x).zip(Field::get_iter(line.p1.y, line.p2.y))
            {
                self[(x, y)] += 1;
            }
        }
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        self.cols * (x - self.min_x) as usize + (y - self.min_y) as usize
    }
}

impl Index<(i32, i32)> for Field {
    type Output = u32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.numbers[self.get_index(index.0, index.1)]
    }
}

impl IndexMut<(i32, i32)> for Field {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        let index = self.get_index(index.0, index.1);
        &mut self.numbers[index]
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .numbers
            .chunks(self.cols)
            .map(|x| x.iter().map(|x| format!("{}", x)).collect::<String>() + "\n")
            .collect();

        write!(f, "{}", s.replace("0", "."))
    }
}

fn parse(filename: &str, diagonal: bool) -> io::Result<Field> {
    let file = fs::read_to_string(filename)?;

    Ok(Field::from_str(&file, diagonal))
}

fn part(field: &Field) -> usize {
    field.numbers.iter().filter(|&&x| x >= 2).count()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let field = parse("inputs/day5", false)?;
    let sol = part(&field);
    println!("Solution part one: {}", sol);

    let field = parse("inputs/day5", true)?;
    let sol = part(&field);
    println!("Solution part two: {}", sol);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let field = parse("inputs/day5_test", false).unwrap();
        println!("{}", field);
        assert_eq!(field.cols, 10);
        assert_eq!(field.rows, 10);
        assert_eq!(field.numbers[0], 0);
    }

    #[test]
    fn test_part_one() {
        let field = parse("inputs/day5_test", false).unwrap();
        println!("{}", field);
        assert_eq!(part(&field), 5);
    }

    #[test]
    fn test_part_two() {
        let field = parse("inputs/day5_test", true).unwrap();
        println!("{}", field);
        assert_eq!(part(&field), 12);
    }
}
