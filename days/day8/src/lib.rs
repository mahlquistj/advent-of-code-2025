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

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    #[inline]
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false; // Already in same set
        }

        // Union by rank
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
            self.rank[px] += 1;
        }
        true
    }

}

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
fn squared_distance(p1: &Position, p2: &Position) -> Number {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let dz = p2.z - p1.z;
    dx * dx + dy * dy + dz * dz
}

pub struct JunctionBoxes {
    // Distance -> index pairs (instead of full Position structs)
    distances: BTreeMap<Number, Vec<(usize, usize)>>,
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

        for i in 0..boxes.len() {
            for j in (i + 1)..boxes.len() {
                let distance = squared_distance(&boxes[i], &boxes[j]);
                distances
                    .entry(distance)
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }

        Ok(Self { distances, boxes })
    }
}

impl JunctionBoxes {
    pub fn find_connections(&self, iterations: usize) -> usize {
        let mut uf = UnionFind::new(self.boxes.len());
        let mut counter = 0;

        'outer: for pairs in self.distances.values() {
            for &(i, j) in pairs {
                uf.union(i, j);
                counter += 1;

                if counter == iterations {
                    break 'outer;
                }
            }
        }

        // Find all unique roots and their component sizes
        let mut sizes: Vec<usize> = (0..self.boxes.len())
            .filter_map(|i| {
                if uf.find(i) == i {
                    Some(uf.size[i])
                } else {
                    None
                }
            })
            .collect();

        sizes.sort_unstable();
        sizes.reverse();

        sizes[0] * sizes[1] * sizes[2]
    }

    pub fn find_last_distance(&self) -> isize {
        let mut uf = UnionFind::new(self.boxes.len());
        let mut last_pair = (0, 0);

        for pairs in self.distances.values() {
            for &(i, j) in pairs {
                if uf.union(i, j) {
                    last_pair = (i, j);
                }
            }
        }

        self.boxes[last_pair.0].x * self.boxes[last_pair.1].x
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
