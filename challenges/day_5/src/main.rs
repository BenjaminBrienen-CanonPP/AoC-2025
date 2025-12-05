use std::{fs, ops::RangeInclusive};

fn main() -> Result<(), String> {
    let input_data = fs::read_to_string("./assets/day_5/input.txt")
        .map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(&input_data)?;
    let result = solve(sequence);
    println!("Result: {result}");
    Ok(())
}

fn solve(sequence: Sequence) -> u64 {
    let mut ranges = sequence.0;
    ranges.sort_by_key(|r| *r.start());

    let mut total = 0u64;
    let mut current: Option<(u64, u64)> = None;

    for r in ranges {
        let start = *r.start();
        let end = *r.end();
        if start > end {
            continue;
        }

        match current {
            None => current = Some((start, end)),
            Some((cs, ce)) => {
                if start <= ce.saturating_add(1) {
                    current = Some((cs, ce.max(end)));
                } else {
                    total = total.saturating_add(ce.saturating_sub(cs).saturating_add(1));
                    current = Some((start, end));
                }
            }
        }
    }

    if let Some((cs, ce)) = current {
        total = total.saturating_add(ce.saturating_sub(cs).saturating_add(1));
    }

    total
}

fn parse_input(input_data: &str) -> Result<Sequence, String> {
    let Some((ranges, ids)) = input_data
        .split_once("\r\n\r\n")
        .or_else(|| input_data.split_once("\n\n"))
    else {
        return Err("invalid input".to_owned());
    };

    let ranges = ranges
        .lines()
        .map(str::trim)
        .filter(|datapoint| !datapoint.is_empty())
        .map(parse_one_id_range)
        .collect::<Result<Vec<_>, String>>()?;

    let ids = ids
        .lines()
        .map(str::trim)
        .filter(|datapoint| !datapoint.is_empty())
        .map(parse_one_id)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "invalid input for ids".to_owned())?;

    Ok((ranges, ids))
}

type Sequence = (Vec<RangeInclusive<u64>>, Vec<u64>);

fn parse_one_id_range(value: &str) -> Result<RangeInclusive<u64>, String> {
    let Some((start, end)) = value.split_once('-') else {
        return Err(format!("invalid value: {value}"));
    };
    let Ok(start) = start.parse::<u64>() else {
        return Err(format!("invalid value: {value}, start: {start}"));
    };
    let Ok(end) = end.parse::<u64>() else {
        return Err(format!("invalid value: {value}, end: {end}"));
    };
    Ok(start..=end)
}

fn parse_one_id(value: &str) -> Option<u64> {
    value.parse().ok()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(
        "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
",
        14
    )]
    fn example_correct(input: &str, expect: u64) {
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
