use std::{num::ParseIntError, ops::RangeInclusive};

pub type Id = u64;
const BASE: Id = 10;

#[derive(Debug)]
pub enum ParseError {
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct IdRange {
    start: Id,
    end: Id,
}

impl IdRange {
    fn as_range(&self) -> RangeInclusive<Id> {
        self.start..=self.end
    }
}

pub struct IdChecker {
    ranges: Vec<IdRange>,
}

impl IdChecker {
    pub fn from_ids(ids: &str) -> Result<Self, ParseError> {
        let ranges = ids
            .trim()
            .split(',')
            .map(|s| {
                let mut split = s.split('-');
                let (start, end) = split.next().zip(split.next()).unwrap();
                Ok(IdRange {
                    start: start.parse()?,
                    end: end.parse()?,
                })
            })
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        Ok(Self { ranges })
    }

    #[inline(always)]
    fn find_invalid_ids_sum<P>(&self, predicate: P) -> Id
    where
        P: Fn(&Id, u64) -> bool + 'static,
    {
        self.ranges
            .iter()
            .flat_map(|id_range| {
                id_range.as_range().filter(|n| {
                    let digits = count_digits(*n);
                    predicate(n, digits)
                })
            })
            .sum::<Id>()
    }

    pub fn sum_angel_numbers(&self) -> Id {
        self.find_invalid_ids_sum(is_angel_number)
    }

    pub fn sum_invalid_ids(&self) -> Id {
        self.find_invalid_ids_sum(is_invalid_id)
    }
}

#[inline(always)]
fn count_digits(n: Id) -> Id {
    n.ilog10() as Id + 1
}

#[inline(always)]
fn is_angel_number(n: &Id, digits: u64) -> bool {
    // If number if digits is uneven, completely ignore it.
    if !digits.is_multiple_of(2) {
        return false;
    }

    let exp = (digits / 2) as u32;
    let base = BASE.pow(exp);
    let pattern = n % base;
    let removed = n - pattern;
    let b_pattern = removed / base;

    pattern == b_pattern
}

#[inline(always)]
fn is_invalid_id(n: &Id, digits: u64) -> bool {
    let max = digits / 2;

    (1..=max).rev().any(|chunk_size| {
        // Avoid uneven splits
        if !digits.is_multiple_of(chunk_size) {
            return false;
        }

        let base = BASE.pow(chunk_size as u32);
        let pattern = n % base;
        let mut remainder = (n - pattern) / base;

        if remainder == 0 {
            return false;
        }

        while remainder != 0 {
            let b_pattern = remainder % base;
            if b_pattern != pattern {
                return false;
            }
            remainder = (remainder - b_pattern) / base;
        }

        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1300), 4);
        assert_eq!(count_digits(130293), 6);
    }

    #[test]
    fn test_invalid_id() {
        assert!(is_invalid_id(&11, 2));
        assert!(is_invalid_id(&22, 2));
        assert!(is_invalid_id(&99, 2));
        assert!(is_invalid_id(&111, 3));
        assert!(is_invalid_id(&999, 3));
        assert!(is_invalid_id(&1010, 4));
        assert!(is_invalid_id(&222222, 6));
        assert!(is_invalid_id(&446446, 6));
        assert!(is_invalid_id(&565656, 6));
        assert!(is_invalid_id(&38593859, 8));
        assert!(is_invalid_id(&824824824, 9));
        assert!(is_invalid_id(&2121212121, 10));
        assert!(is_invalid_id(&1188511885, 10));
    }

    #[test]
    fn solution_1() {
        let checker = IdChecker::from_ids(EXAMPLE1).unwrap();
        assert_eq!(checker.sum_angel_numbers(), 1227775554);
    }

    #[test]
    fn solution_2() {
        let checker = IdChecker::from_ids(EXAMPLE1).unwrap();
        assert_eq!(checker.sum_invalid_ids(), 4174379265);
    }
}
