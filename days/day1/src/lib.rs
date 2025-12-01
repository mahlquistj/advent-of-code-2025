use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum ParseError {
    InvalidCharacter(char),
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

pub struct DialInstructions {
    instructions: Vec<Rotation>,
}

impl DialInstructions {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let instructions: Vec<Rotation> = input
            .split_ascii_whitespace()
            .map(Rotation::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { instructions })
    }

    pub fn apply_to_lock_with_fn<F>(&self, lock: &mut DialLock, mut f: F)
    where
        F: FnMut(i16),
    {
        self.instructions.iter().for_each(|rotation| {
            lock.rotate(rotation);
            f(lock.current);
        });
    }
}

#[derive(Debug)]
pub enum Rotation {
    Right(i16),
    Left(i16),
}

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, amount) = s.split_at(1);
        let amount: i16 = amount.parse()?;

        match instruction {
            "L" => Ok(Self::Left(amount)),
            "R" => Ok(Self::Right(amount)),
            invalid => Err(ParseError::InvalidCharacter(
                invalid.chars().next().unwrap_or(' '),
            )),
        }
    }
}

pub struct DialLock {
    current: i16,
    max: i16,
}

impl DialLock {
    pub fn new(start_at: i16, max: i16) -> Self {
        Self {
            current: start_at,
            max,
        }
    }

    pub fn current(&self) -> i16 {
        self.current
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        match dbg!(rotation) {
            // Right-rotations are addition
            Rotation::Right(amount) => {
                let new = amount + self.current;
                self.current = new % (self.max + 1);
            }
            // Left-rotation are subtraction
            Rotation::Left(amount) => {
                let rem = amount % (self.max + 1);
                self.current -= rem;
                if 0 > self.current {
                    let overflow = self.current.abs_diff(0) as i16;
                    self.current = self.max - (overflow - 1);
                }
            }
        }
        dbg!(self.current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"
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
    "#;

    #[test]
    fn whole_rotations() {
        let mut lock = DialLock::new(50, 99);
        lock.rotate(&Rotation::Right(300));
        assert_eq!(lock.current, 50);
        lock.rotate(&Rotation::Left(300));
        assert_eq!(lock.current, 50);
    }

    #[test]
    fn solution_1() {
        let instructions = DialInstructions::parse(EXAMPLE1).unwrap();
        let mut lock = DialLock::new(50, 99);
        let mut counter = 0;
        instructions.apply_to_lock_with_fn(&mut lock, |num| {
            if num == 0 {
                counter += 1;
            }
        });

        assert_eq!(counter, 3)
    }
}
