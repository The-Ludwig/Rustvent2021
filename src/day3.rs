use std::fmt::format;
use std::fs;
use std::io;

fn part_one(size: usize, bytes: &Vec<u32>) -> (u32, u32) {
    let result = (0..size)
        .map(|i| u32::pow(2, i as u32))
        .map(|mask| {
            (bytes.iter().filter(|b| (*b & mask) > 0).count() > (bytes.len() / 2)) as u32 * mask
        })
        .sum();

    (result, u32::pow(2, size as u32) - 1 - result)
}

fn part_two(size: usize, bytes: &Vec<u32>) -> (u32, u32) {
    let mut oxygen = 0;
    let mut ox_bytes = bytes.clone();
    for i in (0..size).rev() {
        let mask = u32::pow(2, i as u32);
        let len = ox_bytes.iter().filter(|b| (*b & mask) > 0).count();
        let most_common_bit = len * 2 >= ox_bytes.len();

        ox_bytes = ox_bytes
            .into_iter()
            .filter(|b| ((*b & mask) != 0) == most_common_bit)
            .collect();

        if ox_bytes.len() == 1 {
            oxygen = ox_bytes[0];
            break;
        }
    }

    let mut carbon = 0;
    let mut c_bytes = bytes.clone();
    for i in (0..size).rev() {
        let mask = u32::pow(2, i as u32);
        let len = c_bytes.iter().filter(|b| (*b & mask) > 0).count();
        let most_common_bit = len * 2 < c_bytes.len();

        c_bytes = c_bytes
            .into_iter()
            .filter(|b| ((*b & mask) != 0) == most_common_bit)
            .collect();

        if c_bytes.len() == 1 {
            carbon = c_bytes[0];
            break;
        }
    }

    (oxygen, carbon)
}

fn parse(filename: &str) -> io::Result<(usize, Vec<u32>)> {
    let file = fs::read_to_string(filename)?;
    let lines: Vec<_> = file.lines().collect();
    let size = lines.iter().next().unwrap().len();

    Ok((
        size,
        lines
            .iter()
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect(),
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let (size, bytes) = parse("inputs/day3").unwrap();
    let (gamma, epsilon) = part_one(size, &bytes);
    println!(
        "Part 1: γ={} ε={}\nMultiplied: {}\n",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let (oxygen, carbon) = part_two(size, &bytes);
    println!(
        "Part 2: O={} C={}\nMultiplied: {}",
        oxygen,
        carbon,
        oxygen * carbon
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let bytes = parse("inputs/day3_test").unwrap();
        assert_eq!(bytes.1[0], 4);
        assert_eq!(bytes.1[1], 30);
    }

    #[test]
    fn test_part1() {
        let (size, bytes) = parse("inputs/day3_test").unwrap();
        assert_eq!(part_one(size, &bytes), (22, 9));
    }

    #[test]
    fn test_part2() {
        let (size, bytes) = parse("inputs/day3_test").unwrap();
        assert_eq!(part_two(size, &bytes), (23, 10));
    }
}
