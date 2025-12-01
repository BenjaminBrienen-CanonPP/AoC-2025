use std::fs::read_to_string;
use std::{ops::Add, str::FromStr};

fn main() -> Result<(), String> {
    let input_data =
        read_to_string("./assets/day_1/input.txt").map_err(|_| "failed to read file".to_owned())?;
    let sequence = parse_input(input_data)?;
    let result = solve(sequence);
    println!("Result: {result:?}");
    Ok(())
}

fn solve(sequence: Sequence) -> (Rotation, u32) {
    let mut zeroes = 0;
    let ending = sequence.iter().fold(Rotation(50), |accumulator, &new| {
        let next = accumulator + new;
        if next == Rotation(0) {
            zeroes += 1
        }
        next
    });
    (ending, zeroes)
}

fn parse_input(input_data: String) -> Result<Vec<DirectionalRotation>, String> {
    input_data
        .lines()
        .map(|input_line| input_line.trim())
        .filter(|input_line| !input_line.is_empty())
        .map(|input_line| {
            input_line
                .parse::<DirectionalRotation>()
                .map_err(|_| input_line)
        })
        .collect::<Result<Vec<DirectionalRotation>, &str>>()
        .map_err(|invalid_line| format!("invalid input: {invalid_line}"))
}

const DIAL_MAX: u32 = 100;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
struct Rotation(u32);

impl Add<DirectionalRotation> for Rotation {
    type Output = Rotation;

    fn add(self, rhs: DirectionalRotation) -> Self::Output {
        let magnitude = rhs.1 % DIAL_MAX;
        match rhs.0 {
            Direction::Right => Rotation((self.0 + magnitude) % DIAL_MAX),
            Direction::Left => Rotation((self.0 + DIAL_MAX - magnitude) % DIAL_MAX),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
struct DirectionalRotation(Direction, u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
enum Direction {
    Left,
    Right,
}

type Sequence = Vec<DirectionalRotation>;

impl FromStr for DirectionalRotation {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let direction = match value.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => return Err(()),
        };
        let magnitude = value.split_at(1).1.parse::<u32>().map_err(|_| ())?;
        if magnitude == 0 {
            return Err(());
        }
        Ok(Self(direction, magnitude))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, DirectionalRotation, Rotation};

    #[test]
    fn can_parse_valid() {
        let cases = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "L100",
        ];
        for case in cases {
            assert!(
                case.parse::<DirectionalRotation>().is_ok(),
                "case {case} failed"
            );
        }
    }

    #[test]
    fn cannot_parse_invalid() {
        let cases = ["68", "C30", "R-11", "R0", "L00"];
        for case in cases {
            assert!(
                case.parse::<DirectionalRotation>().is_err(),
                "case {case} failed"
            );
        }
    }

    #[test]
    fn directional_rotation_addition_correctness() {
        assert_eq!(
            Rotation(0) + DirectionalRotation(Direction::Right, 0),
            Rotation(0),
            "identity right"
        );
        assert_eq!(
            Rotation(0) + DirectionalRotation(Direction::Right, 1),
            Rotation(1),
            "small right"
        );
        assert_eq!(
            Rotation(99) + DirectionalRotation(Direction::Right, 1),
            Rotation(0),
            "small overflow"
        );
        assert_eq!(
            Rotation(20) + DirectionalRotation(Direction::Right, 20),
            Rotation(40),
            "normal right"
        );
        assert_eq!(
            Rotation(99) + DirectionalRotation(Direction::Right, 99),
            Rotation(98),
            "large overflow"
        );
        assert_eq!(
            Rotation(0) + DirectionalRotation(Direction::Left, 0),
            Rotation(0),
            "identity left"
        );
        assert_eq!(
            Rotation(0) + DirectionalRotation(Direction::Left, 1),
            Rotation(99),
            "small underflow"
        );
        assert_eq!(
            Rotation(99) + DirectionalRotation(Direction::Left, 1),
            Rotation(98),
            "small left"
        );
        assert_eq!(
            Rotation(20) + DirectionalRotation(Direction::Left, 20),
            Rotation(0),
            "normal left"
        );
        assert_eq!(
            Rotation(0) + DirectionalRotation(Direction::Left, 90),
            Rotation(10),
            "large underflow"
        );
        assert_eq!(
            Rotation(11) + DirectionalRotation(Direction::Right, 8),
            Rotation(19),
            "example 1"
        );
        assert_eq!(
            Rotation(19) + DirectionalRotation(Direction::Left, 19),
            Rotation(0),
            "example 2"
        );
    }

    #[test]
    fn solve_example() {
        let parsed = super::parse_input(
            r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#
            .to_owned(),
        )
        .unwrap();
        assert_eq!(super::solve(parsed), (Rotation(32), 3));
    }
}
