use std::{collections::BTreeMap, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum ParseError {
    ParseInt(ParseIntError),
    EmptyPosition,
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

type Number = isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: Number,
    y: Number,
    z: Number,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl FromStr for Position {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|n| n.parse());
        Ok(Self {
            x: iter.next().ok_or(ParseError::EmptyPosition)??,
            y: iter.next().ok_or(ParseError::EmptyPosition)??,
            z: iter.next().ok_or(ParseError::EmptyPosition)??,
        })
    }
}

#[inline(always)]
fn euclidean_distance(p1: &Position, p2: &Position) -> Number {
    ((p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2) + (p2.z - p1.z).pow(2)).isqrt()
}

pub struct JunctionBoxes {
    // Distance -> Position-pairs
    distances: BTreeMap<Number, Vec<[Position; 2]>>,
    boxes: Vec<Position>,
}

impl FromStr for JunctionBoxes {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boxes = s
            .lines()
            .map(Position::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let mut distances = BTreeMap::new();
        let mut counter = 0;

        while counter < boxes.len() {
            let current = &boxes[counter];
            for b in boxes[(counter + 1)..].iter() {
                let distance = euclidean_distance(current, b);
                distances
                    .entry(distance)
                    .and_modify(|pairs: &mut Vec<[Position; 2]>| pairs.push([*current, *b]))
                    .or_insert_with(|| vec![[*current, *b]]);
            }

            counter += 1;
        }

        Ok(Self { distances, boxes })
    }
}

impl JunctionBoxes {
    pub fn find_connections(&self, iterations: usize) -> usize {
        let mut circuits: Vec<Vec<Position>> = self.boxes.iter().map(|b| vec![*b]).collect();
        let mut counter = 0;

        'a: for pairs in self.distances.values() {
            for pair in pairs {
                counter += 1;

                if counter == iterations {
                    break 'a;
                }

                let a_idx = circuits
                    .iter()
                    .enumerate()
                    .find(|(_, circuit)| circuit.contains(&pair[0]))
                    .unwrap()
                    .0;

                let mut a = circuits.remove(a_idx);

                let Some(b) = circuits
                    .iter_mut()
                    .find(|circuit| circuit.contains(&pair[1]))
                else {
                    // Same circuit - Do nothing
                    circuits.push(a);
                    continue;
                };

                b.append(&mut a);
            }
        }

        circuits.sort_by_key(|a| a.len());

        circuits.pop().unwrap().len()
            * circuits.pop().unwrap().len()
            * circuits.pop().unwrap().len()
    }

    pub fn find_last_distance(&self) -> isize {
        let mut circuits: Vec<Vec<Position>> = self.boxes.iter().map(|b| vec![*b]).collect();
        let mut last_connection: [Position; 2] = [Position { x: 0, y: 0, z: 0 }; 2];

        for pairs in self.distances.values() {
            for pair in pairs {
                let a_idx = circuits
                    .iter()
                    .enumerate()
                    .find(|(_, circuit)| circuit.contains(&pair[0]))
                    .unwrap()
                    .0;

                let mut a = circuits.remove(a_idx);

                let Some(b) = circuits
                    .iter_mut()
                    .find(|circuit| circuit.contains(&pair[1]))
                else {
                    // Same circuit - Do nothing
                    circuits.push(a);
                    continue;
                };

                b.append(&mut a);

                last_connection = *pair;
            }
        }

        last_connection[0].x * last_connection[1].x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn solution_1() {
        let boxes = JunctionBoxes::from_str(EXAMPLE).unwrap();
        assert_eq!(boxes.find_connections(10), 40)
    }

    #[test]
    fn solution_2() {
        let boxes = JunctionBoxes::from_str(EXAMPLE).unwrap();
        assert_eq!(boxes.find_last_distance(), 25272)
    }
}
