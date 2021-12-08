use std::fs;

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

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let p1 = part_one("inputs/day8");
    println!("Answer part one: {}", p1);

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
}
