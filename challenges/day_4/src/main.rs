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
    fn get(sequence: &Sequence, y: isize, x: isize) -> Option<Spot> {
        if y < 0 || x < 0 || y >= sequence.len() as isize || x >= sequence[0].len() as isize {
            return None;
        }
        Some(sequence[y as usize][x as usize])
    }
    let start = sequence
        .iter()
        .flatten()
        .filter(|spot| **spot == Spot::Paper)
        .count();
    let mut is_changed = true;
    while is_changed {
        is_changed = false;
        for x in 0..sequence[0].len() {
            for y in 0..sequence.len() {
                let papers = [
                    get(&sequence, y as isize - 1, x as isize - 1),
                    get(&sequence, y as isize - 1, x as isize + 0),
                    get(&sequence, y as isize - 1, x as isize + 1),
                    get(&sequence, y as isize + 0, x as isize - 1),
                    // get(&sequence, y as isize + 0, x as isize + 0),
                    get(&sequence, y as isize + 0, x as isize + 1),
                    get(&sequence, y as isize + 1, x as isize - 1),
                    get(&sequence, y as isize + 1, x as isize + 0),
                    get(&sequence, y as isize + 1, x as isize + 1),
                ]
                .iter()
                .filter(|spot| spot.is_some_and(|spot| spot == Spot::Paper))
                .count();
                // println!("x: {x}, y: {y}, papers: {papers}");
                if sequence[y][x] == Spot::Paper && papers < 4 {
                    // println!("x: {x}, y: {y} is removable");
                    sequence[y][x] = Spot::Empty;
                    is_changed = true;
                }
            }
        }
    }
    let end = sequence
        .iter()
        .flatten()
        .filter(|spot| **spot == Spot::Paper)
        .count();
    (start - end) as u64
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
        43
    )]
    fn example_correct(input: &str, expect: u64) {
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
