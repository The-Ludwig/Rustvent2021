use std::fs;
use std::io;

#[derive(Debug, Clone)]
struct BingoTable {
    numbers: Vec<u32>,
    filled: Vec<bool>,
    row_filled: Vec<u32>,
    col_filled: Vec<u32>,
    pub rows: usize,
    pub cols: usize,
}

impl BingoTable {
    fn from_str(mat: &str) -> BingoTable {
        let lines: Vec<_> = mat.lines().collect();
        let rows = lines.len();
        let cols = lines[0].split(" ").filter(|x| x.len() > 0).count();

        let mut numbers: Vec<u32> = Vec::with_capacity(rows * cols);

        for line in lines {
            let nums = line.split(" ").filter(|x| x.len() > 0);
            for num_str in nums {
                numbers.push(num_str.parse().unwrap());
            }
        }

        let filled = vec![false; numbers.len()];

        BingoTable {
            numbers: numbers,
            filled: filled,
            row_filled: vec![0; rows],
            col_filled: vec![0; cols],
            rows: rows,
            cols: cols,
        }
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    pub fn fill(&mut self, num: u32) -> Option<u32> {
        let index = match self.numbers.iter().position(|x| *x == num) {
            Some(pos) => pos,
            None => return None,
        };

        let (row, col) = self.get_pos(index);

        // now look if we have a bingo in row and col
        if !self.filled[index] {
            self.filled[index] = true;

            self.row_filled[row] += 1;
            self.col_filled[col] += 1;

            if self.row_filled[row] == self.cols as u32 || self.col_filled[col] == self.rows as u32
            {
                let sum: u32 = self
                    .numbers
                    .iter()
                    .zip(self.filled.iter())
                    .filter(|(_, &filled)| !filled)
                    .map(|x| *x.0)
                    .sum();

                return Some(sum * num);
            }
        }

        None
    }

    #[allow(dead_code)]
    fn get(&self, row: usize, col: usize) -> u32 {
        self.numbers[self.cols * row + col]
    }
}

fn part_one(nums: &Vec<u32>, tables: &Vec<BingoTable>) -> u32 {
    let mut tables = (*tables).clone();
    for num in nums {
        for table in tables.iter_mut() {
            match table.fill(*num) {
                Some(x) => return x,
                None => (),
            }
        }
    }

    panic!("No board wins");
}

fn part_two(nums: &Vec<u32>, tables: &Vec<BingoTable>) -> u32 {
    let mut tables = tables.clone();
    let mut won = vec![false; tables.len()];

    for num in nums {
        for (idx, table) in tables.iter_mut().enumerate() {
            if won[idx] {
                continue;
            }
            match table.fill(*num) {
                Some(x) => {
                    if won.iter().filter(|&&x| !x).count() == 1 {
                        return x;
                    } else {
                        won[idx] = true;
                    }
                }
                None => (),
            }
        }
    }

    panic!("No board wins last");
}

fn parse(filename: &str) -> io::Result<(Vec<u32>, Vec<BingoTable>)> {
    let file = fs::read_to_string(filename)?;
    let mut parts = file.split("\n\n");
    let numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let matrices = parts.map(|x| BingoTable::from_str(x)).collect();

    Ok((numbers, matrices))
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let (nums, tables) = parse("inputs/day4")?;

    let res = part_one(&nums, &tables);
    println!("Result of part one is: {}", res);

    let res = part_two(&nums, &tables);
    println!("Result of part two is: {}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_parse() {
        let str =
            " 3 15  0  2 22 \n 9 18 13 17  5 \n19  8  7 25 23 \n20 11 10 24  4 \n14 21 16 12  6";
        let matrix = BingoTable::from_str(str);
        assert_eq!(matrix.rows, 5);
        assert_eq!(matrix.cols, 5);
        assert_eq!(matrix.get(2, 3), 25)
    }

    #[test]
    fn test_parse() {
        let (nums, mat) = parse("inputs/day4_test").unwrap();

        assert_eq!(
            nums,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(mat[0].cols, 5);
        assert_eq!(mat[2].rows, 5);
        assert_eq!(mat[2].get(1, 1), 16);
    }

    #[test]
    fn test_part_one() {
        let (nums, tables) = parse("inputs/day4_test").unwrap();

        let res = part_one(&nums, &tables);
        assert_eq!(res, 4512);
    }

    #[test]
    fn test_part_two() {
        let (nums, tables) = parse("inputs/day4_test").unwrap();

        let res = part_two(&nums, &tables);
        assert_eq!(res, 1924);
    }
}
