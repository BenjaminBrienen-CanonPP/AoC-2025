use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn main() -> Result<(), String> {
    let input_data =
        read_to_string("./assets/day_2/input.txt").map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(input_data)?;
    let result = solve(sequence);
    println!("Result: {result:?}");
    Ok(())
}

fn solve(sequence: Sequence) -> u64 {
    sequence
        .into_iter()
        .map(|range| {
            let mut occurrences = 0;
            for i in range {
                let as_string = i.to_string();
                let length = as_string.len();
                let first_half = &as_string[..length / 2];
                let second_half = &as_string[length / 2..];
                if first_half == second_half {
                    occurrences += i;
                }
            }
            occurrences
        })
        .sum()
}

fn parse_input(input_data: String) -> Result<Vec<RangeInclusive<u64>>, String> {
    input_data
        .split_terminator(',')
        .filter(|datapoint| !datapoint.is_empty())
        .map(|datapoint| datapoint.trim())
        .map(parse_one)
        .collect::<Option<Vec<RangeInclusive<u64>>>>()
        .ok_or_else(|| "invalid input".to_string())
}

type Sequence = Vec<RangeInclusive<u64>>;

fn parse_one(value: &str) -> Option<RangeInclusive<u64>> {
    let dash_index = value.find('-')?;
    let (first, second) = value.split_at(dash_index);
    let (first, second) = (first.parse::<u64>().ok()?, second[1..].parse::<u64>().ok()?);
    Some(first..=second)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_correct() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_owned();
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, 1227775554);
    }
}
