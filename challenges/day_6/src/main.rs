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
    let mut problems: Sequence = Vec::new();
    let mut problem_index: usize = 0;
    let operand_data: Vec<_> = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    for (i, character) in final_line.chars().enumerate() {
        if let Some(operation) = parse_operation(character) {
            problems.push((Vec::new(), operation));
        }
        let mut operand: Operand = 0;
        // luckily, cephalopod math doesn't have zeroes!
        for j in 0..operand_data.len() {
            if let Some(digit) = operand_data[operand_data.len() - j - 1][i].to_digit(10) {
                operand = operand * 10 + digit as Operand;
            }
        }
        if operand == 0 {
            problem_index += 1;
        } else {
            problems[problem_index].0.push(operand);
        }
    }
    Ok(problems)
}

fn parse_operation(character: char) -> Option<Operation> {
    match character {
        '+' => Some(Operation::Addition),
        '*' => Some(Operation::Multiplication),
        _ => None,
    }
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
        3263827
    )]
    fn example_correct(input: &str, expect: Operand) {
        let parsed = super::parse_input(input).unwrap();
        let solution = super::solve(parsed);
        assert_eq!(solution, expect);
    }
}
