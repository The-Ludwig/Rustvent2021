use std::fs;
use std::io;

enum Instruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Instruction {
    fn from_string(string: &str) -> Instruction {
        let mut parts = string.split(' ');
        let inst = parts.next().expect("Line not long enough");
        let number = parts
            .next()
            .expect("Line not long enough")
            .parse::<i32>()
            .expect("Second part of line has to be a number");

        match inst {
            "up" => Instruction::Up(number),
            "down" => Instruction::Down(number),
            "forward" => Instruction::Forward(number),
            _ => panic!("Unknown instruction {}", inst),
        }
    }
}

fn part_one(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut depth = 0;
    let mut position = 0;
    for i in instructions {
        match i {
            Instruction::Up(x) => depth -= x,
            Instruction::Down(x) => depth += x,
            Instruction::Forward(x) => position += x,
        }
    }

    (depth, position)
}

fn part_two(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    let mut aim: i32 = 0;

    for i in instructions {
        match i {
            Instruction::Up(x) => aim -= x,
            Instruction::Down(x) => aim += x,
            Instruction::Forward(x) => {
                position += x;
                depth += aim * x;
            }
        }
    }

    (depth, position)
}

fn parse(filename: &str) -> io::Result<Vec<Instruction>> {
    Ok(fs::read_to_string(filename)?
        .lines()
        .map(|x| Instruction::from_string(x))
        .collect())
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let instructions = parse("inputs/day2")?;

    let (depth, position) = part_one(&instructions);
    println!("Depth, position part1: {}, {}", depth, position);
    println!("Multiplied: {}\n", depth * position);

    let (depth, position) = part_two(&instructions);
    println!("Depth, position part2: {}, {}", depth, position);
    println!("Multiplied: {}", depth * position);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let instructions = parse("inputs/day2_test").unwrap();

        assert_eq!(part_one(&instructions), (10, 15));
        assert_eq!(part_two(&instructions), (60, 15));
    }
}
