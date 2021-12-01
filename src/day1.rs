use std::{fs, vec};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("inputs/day1")?;

    let values: Vec<_> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut sum = 0;
    let mut iter = values.iter();
    let mut current = iter.next().unwrap();
    for num in iter {
        if num > current {
            sum += 1;
        }
        current = num;
    }
    println!("Answer Part one: '{}'", sum);

    let len = values.len();
    let mut iter = values[..len - 2]
        .iter()
        .zip(&values[1..len - 1])
        .zip(&values[2..])
        .map(|x| x.0 .0 + x.0 .1 + x.1);

    let mut sum = 0;
    let mut current = iter.next().unwrap();
    for num in iter {
        if num > current {
            sum += 1;
        }
        current = num;
    }
    println!("Answer Part two: '{}'", sum);

    Ok(())
}
