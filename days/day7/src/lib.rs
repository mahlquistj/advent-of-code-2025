use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
pub enum ParseError {
    NoInput,
}

pub struct TachyonManifold {
    input: usize,
    splitters: Vec<usize>,
}

impl FromStr for TachyonManifold {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let input = lines
            .next()
            .and_then(|line| {
                line.char_indices()
                    .find(|(_, c)| *c == 'S')
                    .map(|(column, _)| column)
            })
            .ok_or(ParseError::NoInput)?;

        let splitters = lines
            .flat_map(|line| {
                line.char_indices().filter_map(
                    move |(column, c)| {
                        if c == '^' { Some(column) } else { None }
                    },
                )
            })
            .collect();

        Ok(Self { input, splitters })
    }
}

impl TachyonManifold {
    pub fn calculate_beam_splits(&self) -> usize {
        let mut splits = 0;
        let mut current_beams = HashSet::new();
        current_beams.insert(self.input);

        for splitter in &self.splitters {
            if current_beams.remove(splitter) {
                current_beams.insert(splitter.saturating_add(1));
                current_beams.insert(splitter.saturating_sub(1));
                splits += 1
            }
        }

        splits
    }

    pub fn calculate_beam_timelines(&self) -> usize {
        let mut current_beams = HashMap::new();
        current_beams.insert(self.input, 1);

        for splitter in &self.splitters {
            if let Some(beams) = current_beams.remove(splitter) {
                current_beams
                    .entry(splitter.saturating_add(1))
                    .and_modify(|b| *b += beams)
                    .or_insert(beams);
                current_beams
                    .entry(splitter.saturating_sub(1))
                    .and_modify(|b| *b += beams)
                    .or_insert(beams);
            }
        }

        current_beams.drain().map(|(_, b)| b).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    const CASE: &str = r#"...S...
.......
...^...
.......
..^.^..
.......
.^.^.^."#;

    #[test]
    fn case() {
        let manifold = TachyonManifold::from_str(CASE).unwrap();
        assert_eq!(manifold.calculate_beam_timelines(), 8);
    }

    #[test]
    fn solution_1() {
        let manifold = TachyonManifold::from_str(EXAMPLE).unwrap();
        assert_eq!(manifold.calculate_beam_splits(), 21);
    }

    #[test]
    fn solution_2() {
        let manifold = TachyonManifold::from_str(EXAMPLE).unwrap();
        assert_eq!(manifold.calculate_beam_timelines(), 40);
    }
}
