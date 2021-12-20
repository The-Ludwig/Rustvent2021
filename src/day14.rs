use std::collections::BTreeMap;
use std::{fs, io};

fn parse(filename: &str) -> io::Result<(String, BTreeMap<(char, char), char>)> {
    let file = fs::read_to_string(filename)?;
    let mut parts = file.split("\n\n");

    let start = parts.next().unwrap().to_string();
    let replacements = parts
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            let mut p = x.split(" -> ");
            let mut pat = p.next().unwrap().chars();
            (
                (pat.next().unwrap(), pat.next().unwrap()),
                p.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect();

    Ok((start, replacements))
}

fn part(start: &String, instructions: &BTreeMap<(char, char), char>, n: usize) -> usize {
    let mut state = BTreeMap::new();
    let mut counts = BTreeMap::new();

    let chars: Vec<char> = start.chars().collect();
    for c in chars.windows(2) {
        *state.entry((c[0], c[1])).or_insert(0) += 1;
    }

    for c in chars {
        *counts.entry(c).or_insert(0) += 1;
    }

    for _ in 0..n {
        let save_state = state.clone();
        for (pair, count) in save_state {
            match instructions.get(&pair) {
                Some(c) => {
                    *state.entry((pair.0, pair.1)).or_insert(0) -= count;
                    *state.entry((pair.0, *c)).or_insert(0) += count;
                    *state.entry((*c, pair.1)).or_insert(0) += count;
                    *counts.entry(*c).or_insert(0) += count;
                }
                None => (),
            }
        }
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let (start, instructions) = parse("inputs/day14")?;
    println!("Answer Part One: {}", part(&start, &instructions, 10));
    println!("Answer Part Two: {}", part(&start, &instructions, 40));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (start, instructions) = parse("inputs/day14_test").unwrap();
        assert!(start == "NNCB");
        assert!(instructions[&('C', 'H')] == 'B');
        assert!(instructions[&('C', 'N')] == 'C');
    }

    #[test]
    fn test_part_one() {
        let (start, instructions) = parse("inputs/day14_test").unwrap();
        assert_eq!(part(&start, &instructions, 10), 1588);
    }

    #[test]
    fn test_part_two() {
        let (start, instructions) = parse("inputs/day14_test").unwrap();
        assert_eq!(part(&start, &instructions, 40), 2188189693529);
    }
}
