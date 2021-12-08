use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Code {
    digits: Vec<HashSet<char>>,
}

impl Code {
    pub const DIGITS_LENS: [(usize, usize); 4] = [(2, 1), (4, 4), (3, 7), (7, 8)];

    #[allow(dead_code)]
    fn empty() -> Code {
        Code {
            digits: vec![HashSet::new(); 10],
        }
    }

    fn new(training_data: &str) -> Code {
        let digit_lens = HashMap::from(Self::DIGITS_LENS);
        let train_digits: Vec<HashSet<char>> = training_data
            .split(" ")
            .map(|x| HashSet::from_iter(x.chars()))
            .collect();
        let mut digits: Vec<Option<HashSet<char>>> = vec![None; 10];

        let mut six_d: Vec<&HashSet<char>> = Vec::new();
        let mut five_d: Vec<&HashSet<char>> = Vec::new();
        // look at what we can find out with the length of the segments
        for t_digit in &train_digits {
            match t_digit.len() {
                6 => six_d.push(t_digit),
                5 => five_d.push(t_digit),
                len => digits[*digit_lens.get(&len).unwrap()] = Some(t_digit.clone()),
            }
        }

        for sd in six_d {
            if sd.intersection(&digits[1].as_ref().unwrap()).count() == 1 {
                digits[6] = Some(sd.clone());
            } else if sd.intersection(&digits[4].as_ref().unwrap()).count() == 4 {
                digits[9] = Some(sd.clone());
            } else {
                digits[0] = Some(sd.clone());
            }
        }

        for fd in five_d {
            if fd.intersection(&digits[1].as_ref().unwrap()).count() == 2 {
                digits[3] = Some(fd.clone());
            } else if fd.intersection(&digits[4].as_ref().unwrap()).count() == 2 {
                digits[2] = Some(fd.clone());
            } else {
                digits[5] = Some(fd.clone());
            }
        }

        Code {
            digits: digits.into_iter().map(|f| f.unwrap()).collect(),
        }
    }

    fn decode(&self, chars: &str) -> Option<usize> {
        let digits: Vec<HashSet<char>> = chars
            .split(" ")
            .map(|x| HashSet::from_iter(x.chars()))
            .collect();

        let mut result = 0;
        for (idx, charset) in digits.iter().rev().enumerate() {
            let mut found_match = false;
            for (digit, digitset) in self.digits.iter().enumerate() {
                if *charset == *digitset {
                    if found_match {
                        return None;
                    } else {
                        found_match = true;
                        result += digit * usize::pow(10, idx as u32);
                    }
                }
            }
        }

        Some(result)
    }
}

// part one without struct logic, since it will work fine like this
fn part_one(filename: &str) -> usize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| {
            let mut iter = x.split(" | ");
            iter.next();
            let s = iter.next().unwrap();
            s.split(" ")
                .map(|x| x.len())
                .filter(|len| match len {
                    4 => true,
                    3 => true,
                    2 => true,
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

// part one without struct logic, since it will work fine like this
fn part_two(filename: &str) -> usize {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| {
            let mut iter = x.split(" | ");
            let code = Code::new(iter.next().unwrap());
            code.decode(iter.next().unwrap()).unwrap()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let p1 = part_one("inputs/day8");
    println!("Answer part one: {}", p1);

    let p2 = part_two("inputs/day8");
    println!("Answer part two: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let p1 = part_one("inputs/day8_test");
        assert_eq!(p1, 26);
    }

    #[test]
    fn test_part_two() {
        let p2 = part_two("inputs/day8_test");
        assert_eq!(p2, 61229);
    }
}
