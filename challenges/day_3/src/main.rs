use std::fs::read_to_string;

fn main() -> Result<(), String> {
    let input_data =
        read_to_string("./assets/day_3/input.txt").map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(input_data)?;
    let result = solve(sequence);
    println!("Result: {result:?}");
    Ok(())
}

fn solve(sequence: Sequence) -> u64 {
    sequence
        .into_iter()
        .map(|bank| {
            let greatest = bank.iter().take(bank.len() - 1).max().unwrap();
            let index = bank.iter().position(|x| x == greatest).unwrap();
            let second_greatest = bank.iter().skip(index + 1).max().unwrap();
            (greatest * 10 + second_greatest) as u64
        })
        .sum()
}

fn parse_input(input_data: String) -> Result<Sequence, String> {
    input_data
        .lines()
        .map(|datapoint| datapoint.trim())
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

    #[test_case("987654321111111", 98)]
    #[test_case("811111111111119", 89)]
    #[test_case("234234234234278", 78)]
    #[test_case("818181911112111", 92)]
    fn example_correct(input: &str, expect: u64) {
        let parsed = super::parse_input(input.to_string()).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
