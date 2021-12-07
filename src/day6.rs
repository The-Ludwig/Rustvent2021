use std::fs;

fn parse(filename: &str) -> std::io::Result<Vec<usize>> {
    let mut states = vec![0; 9];
    for i in fs::read_to_string(filename)?
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
    {
        states[i] += 1;
    }

    Ok(states)
}

fn step(fish: &mut Vec<usize>) {
    let mut save = fish[0];
    for i in (0..9).rev() {
        let other_save = fish[i];
        fish[i] = save;
        save = other_save;
    }
    fish[6] += save;
}

fn n_step(fish: &mut Vec<usize>, n: u32) {
    for _ in 0..n {
        step(fish);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut fish = parse("inputs/day6")?;
    n_step(&mut fish, 80);
    println!("Answer part 1: {}", fish.iter().sum::<usize>());

    n_step(&mut fish, 256 - 80);
    println!("Answer part 2: {}", fish.iter().sum::<usize>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = parse("inputs/day6_test").unwrap();
        assert_eq!(input, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn test_step() {
        let mut input = parse("inputs/day6_test").unwrap();
        step(&mut input);
        assert_eq!(input, vec![1, 1, 2, 1, 0, 0, 0, 0, 0]);
        step(&mut input);
        assert_eq!(input, vec![1, 2, 1, 0, 0, 0, 1, 0, 1]);
    }

    #[test]
    fn test_part_one() {
        let mut fish = parse("inputs/day6_test").unwrap();
        n_step(&mut fish, 80);
        assert_eq!(fish.iter().sum::<usize>(), 5934);
    }

    #[test]
    fn test_part_two() {
        let mut fish = parse("inputs/day6_test").unwrap();
        n_step(&mut fish, 256);
        assert_eq!(fish.iter().sum::<usize>(), 26984457539);
    }
}
