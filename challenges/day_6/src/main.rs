use std::{fs, ops};

fn main() -> Result<(), String> {
    let input_data = fs::read_to_string("./assets/day_6/input.txt")
        .map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(&input_data)?;
    let result = solve(sequence);
    println!("Result: {result}");
    Ok(())
}

fn solve(sequence: Sequence) -> u64 {
    sequence
        .iter()
        .map(|problem| {
            problem.0.iter().fold(
                match problem.1 {
                    Operation::Addition => 0,
                    Operation::Multiplication => 1,
                },
                match problem.1 {
                    Operation::Addition => ops::Add::add,
                    Operation::Multiplication => ops::Mul::mul,
                },
            )
        })
        .sum()
}

fn parse_input(input_data: &str) -> Result<Sequence, String> {
    let mut lines = input_data.lines().rev();
    let final_line = lines.next().expect("the input data has a last line");
    let operators: Vec<_> = final_line
        .split_whitespace()
        .map(|split| match split.trim() {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => unimplemented!(),
        })
        .collect();
    let problems_count = operators.len();
    let mut problems: Sequence = Vec::with_capacity(problems_count);
    for (i, line) in lines
        .inspect(|line| println!("line: {line}"))
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        let splits: Vec<_> = line
            .split_whitespace()
            .inspect(|split| println!("split: {split}"))
            .collect();
        for j in 0..problems_count {
            if i == 0 {
                problems.push((Vec::new(), operators[j]));
            }
            problems[j]
                .0
                .push(splits[j].parse().expect("input operands are numbers"));
        }
    }
    Ok(problems)
}

type Sequence = Vec<(Vec<Operand>, Operation)>;
type Operand = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Addition,
    Multiplication,
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::Operand;

    #[test_case(
        "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
",
        4277556
    )]
    fn example_correct(input: &str, expect: Operand) {
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
