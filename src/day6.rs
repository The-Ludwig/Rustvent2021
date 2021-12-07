use std::fs;

fn parse(filename: &str) -> std::io::Result<Vec<u8>> {
    Ok(fs::read_to_string(filename)?
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect())
}

fn step_loop(fish: &mut Vec<u8>) {
    let mut n_to_add = 0;

    for i in fish.iter_mut() {
        if *i > 0 {
            *i -= 1
        } else {
            *i = 6;
            n_to_add += 1;
        }
    }

    for _ in 0..n_to_add {
        fish.push(8);
    }
}

fn n_step(fish: &mut Vec<u8>, n: u32) {
    for _ in 0..n {
        step_loop(fish);
    }
}

/// At least efficient, if we only have 8 numbers ;)
fn n_step_efficient(fish: &mut Vec<u8>, n: u32) {
    let mut nums = vec![0u32; 9];
    for f in fish {
        nums[*f as usize] += 1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut fish = parse("inputs/day6")?;
    n_step(&mut fish, 80);
    println!("Answer part 1: {}", fish.len());

    n_step(&mut fish, 10);
    println!("Answer part 2: {}", fish.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = parse("inputs/day6_test").unwrap();
        assert_eq!(input, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_step() {
        let mut fish = parse("inputs/day6_test").unwrap();
        n_step(&mut fish, 1);
        assert_eq!(fish, vec![2, 3, 2, 0, 1]);
    }

    #[test]
    fn test_part_one() {
        let mut fish = parse("inputs/day6_test").unwrap();
        n_step(&mut fish, 18);
        assert_eq!(
            fish,
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        );
        n_step(&mut fish, 62);
        assert_eq!(fish.len(), 5934);
    }

    #[test]
    fn test_part_two() {
        let mut fish = parse("inputs/day6_test").unwrap();
        n_step(&mut fish, 10);
        assert_eq!(fish.len(), 26984457539);
    }
}
