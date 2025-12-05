use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum ParseError {
    MissingRange,
    InvalidRange,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

pub type IngredientId = u64;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct IdRange {
    start: IngredientId,
    end: IngredientId,
}

impl IdRange {
    pub fn contains(&self, id: IngredientId) -> bool {
        self.start <= id && self.end >= id
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }

    pub fn extend(&mut self, other: &Self) {
        *self = Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        };
    }

    pub fn span(&self) -> IngredientId {
        self.start.abs_diff(self.end) + 1
    }
}

pub struct Database {
    ranges: BTreeSet<IdRange>,
    ids: Vec<IngredientId>,
}

impl FromStr for Database {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut ranges = BTreeSet::new();
        loop {
            let line = lines.next().ok_or(ParseError::MissingRange)?;

            if line.trim().is_empty() {
                break;
            }

            let mut split = line.split('-');
            let start = split.next().ok_or(ParseError::InvalidRange)?.parse()?;
            let end = split.next().ok_or(ParseError::InvalidRange)?.parse()?;
            ranges.insert(IdRange { start, end });
        }

        let mut ids = vec![];
        for id in lines {
            ids.push(id.trim().parse()?);
        }

        Ok(Self { ranges, ids })
    }
}

impl Database {
    pub fn is_fresh(&self, id: &IngredientId) -> bool {
        for range in &self.ranges {
            if range.contains(*id) {
                return true;
            }
        }

        false
    }

    pub fn count_fresh_ingredients(&self) -> usize {
        self.ids.iter().filter(|id| self.is_fresh(id)).count()
    }

    pub fn count_fresh_ids(&self) -> IngredientId {
        let mut counter = 0;

        let mut current = *self.ranges.first().unwrap();

        for range in self.ranges.iter().skip(1) {
            if current.intersects(range) {
                current.extend(range);
                continue;
            }

            counter += current.span();
            current = *range;
        }

        counter + current.span()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn solution_1() {
        let database = Database::from_str(EXAMPLE).unwrap();
        assert_eq!(database.count_fresh_ingredients(), 3)
    }

    #[test]
    fn solution_2() {
        let database = Database::from_str(EXAMPLE).unwrap();
        assert_eq!(database.count_fresh_ids(), 14)
    }
}
