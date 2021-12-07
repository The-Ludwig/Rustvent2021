use std::fs;

fn parse(filename: &str) -> std::io::Result<Vec<usize>> {
    Ok(fs::read_to_string(filename)?
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect())
}

fn part_one(pos: &Vec<usize>) -> usize {
    let max = *pos.iter().max().unwrap();
    (0..max)
        .map(|i| pos.iter().map(move |&j| i.max(j) - i.min(j)).sum())
        .min()
        .unwrap()
}

fn part_two(pos: &Vec<usize>) -> usize {
    let max = *pos.iter().max().unwrap();
    let ret: usize = (0..max)
        .map(|i| {
            pos.iter()
                .map(move |&j| {
                    let n = i.max(j) - i.min(j);
                    return n * (n + 1);
                })
                .sum()
        })
        .min()
        .unwrap();
    ret / 2
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = parse("inputs/day7").unwrap();

    println!("Answer part one: {}", part_one(&input));
    println!("Answer part two: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = parse("inputs/day7_test").unwrap();
        assert_eq!(input, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_part_one() {
        let input = parse("inputs/day7_test").unwrap();
        assert_eq!(part_one(&input), 37);
    }

    #[test]
    fn test_part_two() {
        let input = parse("inputs/day7_test").unwrap();
        assert_eq!(part_two(&input), 168);
    }
}
