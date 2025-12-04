use std::cmp::Reverse;
use std::fs;

fn main() -> Result<(), String> {
    let input_data = fs::read_to_string("./assets/day_4/input.txt")
        .map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(&input_data)?;
    let result = solve(sequence);
    println!("Result: {result}");
    Ok(())
}

fn solve(mut sequence: Sequence) -> u64 {
    let padding = vec![Spot::Empty; sequence.len()];
    sequence.insert(0, padding.clone());
    sequence.push(padding);
    sequence
        .windows(3)
        .map(|x| {
            let mut above = x[0].clone();
            let mut middle = x[1].clone();
            let mut below = x[2].clone();
            let mut result: u64 = 0;
            above.insert(0, Spot::Empty);
            above.push(Spot::Empty);
            middle.insert(0, Spot::Empty);
            middle.push(Spot::Empty);
            below.insert(0, Spot::Empty);
            below.push(Spot::Empty);
            for spot in 1..middle.len() - 1 {
                if middle[spot] == Spot::Paper
                    && [
                        above[spot - 1],
                        above[spot],
                        above[spot + 1],
                        middle[spot - 1],
                        middle[spot + 1],
                        below[spot - 1],
                        below[spot],
                        below[spot + 1],
                    ]
                    .iter()
                    .filter(|spot| **spot == Spot::Paper)
                    .count()
                        < 4
                {
                    result += 1;
                }
            }
            result
        })
        .sum()
}

fn parse_input(input_data: &str) -> Result<Sequence, String> {
    input_data
        .lines()
        .map(str::trim)
        .filter(|datapoint| !datapoint.is_empty())
        .map(parse_one)
        .collect::<Option<_>>()
        .ok_or_else(|| "invalid input".to_string())
}

type Sequence = Vec<Vec<Spot>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spot {
    Empty,
    Paper,
}

fn parse_one(value: &str) -> Option<Vec<Spot>> {
    value
        .chars()
        .map(|character| match character {
            '.' => Some(Spot::Empty),
            '@' => Some(Spot::Paper),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(
        "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
        13
    )]
    fn example_correct(input: &str, expect: u64) {
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
