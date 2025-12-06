use std::{num::ParseIntError, str::FromStr};

type Number = u64;

#[derive(Debug)]
pub enum ParseError {
    InvalidInstruction(String),
    EmptyInput,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

pub enum Instruction {
    Add,
    Multiply,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "*" => Ok(Self::Multiply),
            "+" => Ok(Self::Add),
            invalid => Err(ParseError::InvalidInstruction(invalid.to_string())),
        }
    }
}

impl Instruction {
    pub fn solve(&self, a: Number, b: Number) -> Number {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

pub struct Worksheet {
    vertical_numbers: Vec<Vec<Number>>,
    horizontal_numbers: Vec<Vec<Number>>,
    instructions: Vec<Instruction>,
}

impl FromStr for Worksheet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().collect::<Vec<_>>();

        let instructions = lines
            .pop()
            .ok_or(ParseError::EmptyInput)?
            .split_ascii_whitespace()
            .map(Instruction::from_str)
            .collect::<Result<Vec<_>, ParseError>>()?;

        let horizontal_numbers = lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|num| num.parse::<Number>())
                    .collect::<Result<Vec<_>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<_>>, ParseIntError>>()?;

        let columns = lines.first().map(|numbers| numbers.len()).unwrap_or(0);

        let vertical_numbers = (0..columns)
            .filter_map(|col| {
                let mut numbers: Vec<Number> = vec![];

                for line in &lines {
                    let mut number = String::new();

                    let trimmed = number.trim();

                    if trimmed.is_empty() {
                        break;
                    } else {
                        match trimmed.parse() {
                            Ok(n) => numbers.push(n),
                            Err(error) => return Some(Err(error)),
                        }
                    }
                }

                if numbers.is_empty() {
                    None
                } else {
                    Some(Ok(numbers))
                }
            })
            .collect::<Result<Vec<Vec<Number>>, ParseIntError>>()?;

        Ok(Self {
            vertical_numbers,
            horizontal_numbers,
            instructions,
        })
    }
}

impl Worksheet {
    pub fn solve_horizontal_problems_and_sum(&self) -> Number {
        self.instructions
            .iter()
            .enumerate()
            .map(|(col, instruction)| {
                let init = if let Instruction::Add = instruction {
                    0
                } else {
                    1
                };
                self.horizontal_numbers
                    .iter()
                    .fold(init, |acc, numbers| instruction.solve(acc, numbers[col]))
            })
            .sum()
    }

    pub fn solve_vertical_problems_and_sum(&self) -> Number {
        const BASE: Number = 10;

        self.instructions
            .iter()
            .enumerate()
            .map(|(col, instruction)| {
                let mut numbers = vec![];

                self.vertical_numbers.iter().rev().for_each(|row| {
                    let mut number = row[col];
                    let mut digit_idx = 0;

                    println!("Parsing: {number}");

                    while number != 0 {
                        let n = number % BASE;
                        if let Some((digit, exp)) = numbers.get_mut(digit_idx as usize) {
                            *exp *= BASE;
                            // *digit += n;
                            let x = dbg!(*digit + (n * *exp));
                            *digit = x;
                        } else {
                            println!("Pushing: {n}");
                            numbers.push((n, BASE));
                        };

                        number /= BASE;
                        digit_idx += 1;
                        println!("Numbers: {numbers:?}")
                    }

                    println!("Numbers: {numbers:?}")
                });

                println!("Final: numbers: {numbers:?}");

                let init = if let Instruction::Add = instruction {
                    0
                } else {
                    1
                };

                dbg!(
                    numbers
                        .into_iter()
                        .fold(init, |acc, (number, _)| instruction.solve(acc, number))
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn solution_1() {
        let sheet = Worksheet::from_str(EXAMPLE).unwrap();
        assert_eq!(sheet.solve_horizontal_problems_and_sum(), 4277556)
    }

    #[test]
    fn solution_2() {
        let sheet = Worksheet::from_str(EXAMPLE).unwrap();
        assert_eq!(sheet.solve_vertical_problems_and_sum(), 3263827)
    }
}
