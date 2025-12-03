use std::fs;

fn main() -> Result<(), String> {
    let input_data = fs::read_to_string("./assets/day_3/input.txt")
        .map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(&input_data)?;
    let result = solve(sequence);
    println!("Result: {result:?}");
    Ok(())
}

fn solve(sequence: Sequence) -> u64 {
    sequence
        .into_iter()
        .map(|bank| {
            let times = 12;
            let mut result = 0u64;
            let mut last_index = None;
            for i in 0..times {
                let skip = last_index.map_or(0, |index| index + 1);
                let greatest = bank
                    .clone()
                    .into_iter()
                    .skip(skip)
                    .take(
                        (bank.len() - skip) // remaining possible
                        - (times - i)  // needed remaining = times - i
                         + 1, // lookahead
                    )
                    .max()
                    .unwrap();
                last_index = Some(
                    bank.clone()
                        .into_iter()
                        .skip(skip)
                        .position(|x| x == greatest)
                        .unwrap()
                        + skip,
                );
                result *= 10;
                result += greatest as u64;
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
        .collect::<Option<Sequence>>()
        .ok_or_else(|| "invalid input".to_string())
}

type Sequence = Vec<Vec<u8>>;

fn parse_one(value: &str) -> Option<Vec<u8>> {
    value
        .chars()
        .map(|character| character.to_digit(10).map(|x| x as u8))
        .collect::<Option<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case("987654321111111", 987654321111)]
    #[test_case("811111111111119", 811111111119)]
    #[test_case("234234234234278", 434234234278)]
    #[test_case("818181911112111", 888911112111)]
    fn example_correct(input: &str, expect: u64) {
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
