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
        F: FnMut(i16, i16),
    {
        println!("INIT: {lock:#?}");
        self.instructions.iter().for_each(|rotation| {
            let resets = lock.rotate(rotation);
            let current = lock.current;
            println!("{rotation:?} | Lock: {current} | Resets: {resets}");
            f(current, resets);
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

#[derive(Debug)]
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

    // Rotates the lock and returns the amount of times the lock rotated PAST zero
    pub fn rotate(&mut self, rotation: &Rotation) -> i16 {
        let mut resets = 0;
        let started_at = self.current;

        match rotation {
            // Right-rotations are addition
            Rotation::Right(amount) => {
                // Add full-rotations to resets
                resets += amount / (self.max + 1);

                // Add remainder after rotations to current value
                let rem = amount % (self.max + 1);
                self.current += rem;

                if self.max < self.current {
                    // Make sure we only are equal to the remainder of the overflow
                    self.current %= self.max + 1;

                    // If we went over zero, then add another reset
                    if self.current != 0 {
                        resets += 1;
                    }
                }
            }
            // Left-rotation are subtraction
            Rotation::Left(amount) => {
                // Add full rotations to resets
                resets += amount / (self.max + 1);

                // Subtract remainder after rotations from the current value
                let rem = amount % (self.max + 1);
                self.current -= rem;

                if 0 > self.current {
                    // Make sure we only are equal to the remainder of the underflow
                    let overflow = self.current.abs_diff(0) as i16;
                    self.current = self.max - (overflow - 1);

                    // Add another reset
                    if self.current != 0 && started_at != 0 {
                        resets += 1;
                    }
                }
            }
        }

        resets
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
    fn full_rotations() {
        let mut lock = DialLock::new(10, 99);
        assert_eq!(lock.rotate(&Rotation::Right(300)), 3);
        assert_eq!(lock.current, 10);
        assert_eq!(lock.rotate(&Rotation::Left(300)), 3);
        assert_eq!(lock.current, 10);
    }

    #[test]
    fn from_zero() {
        let mut lock = DialLock::new(0, 99);
        assert_eq!(lock.rotate(&Rotation::Right(10)), 0);
        assert_eq!(lock.current, 10);

        let mut lock = DialLock::new(0, 99);
        assert_eq!(lock.rotate(&Rotation::Left(10)), 0);
        assert_eq!(lock.current, 90);
    }

    #[test]
    fn past_zero() {
        let mut lock = DialLock::new(10, 99);
        assert_eq!(lock.rotate(&Rotation::Left(20)), 1);
        assert_eq!(lock.current, 90);
        assert_eq!(lock.rotate(&Rotation::Right(20)), 1);
        assert_eq!(lock.current, 10);
    }

    #[test]
    fn full_rotation_then_past_zero() {
        let mut lock = DialLock::new(50, 99);
        assert_eq!(lock.rotate(&Rotation::Right(155)), 2);
        assert_eq!(lock.current, 5);

        let mut lock = DialLock::new(50, 99);
        assert_eq!(lock.rotate(&Rotation::Left(155)), 2);
        assert_eq!(lock.current, 95);
    }

    #[test]
    fn full_rotation_then_to_zero() {
        let mut lock = DialLock::new(50, 99);
        assert_eq!(lock.rotate(&Rotation::Left(150)), 1);
        assert_eq!(lock.current, 0);

        let mut lock = DialLock::new(50, 99);
        assert_eq!(lock.rotate(&Rotation::Right(150)), 1);
        assert_eq!(lock.current, 0)
    }

    #[test]
    fn solution_1() {
        let instructions = DialInstructions::parse(EXAMPLE1).unwrap();
        let mut lock = DialLock::new(50, 99);
        let mut counter = 0;
        instructions.apply_to_lock_with_fn(&mut lock, |num, _resets| {
            if num == 0 {
                counter += 1;
            }
        });

        assert_eq!(counter, 3)
    }

    #[test]
    fn solution_2() {
        let instructions = DialInstructions::parse(EXAMPLE1).unwrap();
        let mut lock = DialLock::new(50, 99);
        let mut counter = 0;

        instructions.apply_to_lock_with_fn(&mut lock, |num, resets| {
            counter += resets;

            if num == 0 {
                counter += 1;
            }

            println!("Count: {counter}")
        });

        assert_eq!(counter, 6);
    }
}
