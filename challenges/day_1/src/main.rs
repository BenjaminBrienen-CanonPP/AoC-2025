use std::{fs, str::FromStr};

fn main() -> Result<(), ()> {
    Ok(())
}

struct RotationMagnitude(u8);

impl RotationMagnitude {
    pub fn new_from_i8(value: u8) -> Option<Self> {
        match value {
            0..=99 => Some(Self(value)),
            _ => None,
        }
    }
}

struct RotationOffset(Direction, RotationMagnitude);

enum Direction {
    Left,
    Right,
}

type Sequence = Vec<RotationOffset>;

impl FromStr for RotationOffset {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let direction = match value.chars().nth(0) {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => return Err(()),
        };
        let as_u8 = value.split_at(1).1.parse::<u8>().map_err(|_| ())?;
        let magnitude = RotationMagnitude::new_from_i8(as_u8).ok_or_else(|| ())?;
        Ok(RotationOffset(direction, magnitude))
    }
}

#[cfg(test)]
mod tests {
    use crate::RotationOffset;

    #[test]
    fn can_parse_valid() {
        let cases = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "L00",
        ];
        for case in cases {
            assert!(case.parse::<RotationOffset>().is_ok(), "case {case} failed");
        }
    }

    #[test]
    fn cannot_parse_invalid() {
        let cases = [
            "68", "C30", "R-11", "L100"
        ];
        for case in cases {
            assert!(case.parse::<RotationOffset>().is_err(), "case {case} failed");
        }
    }
}
