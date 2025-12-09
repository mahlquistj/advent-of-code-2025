use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum ParseError {
    ParseInt(ParseIntError),
    InvalidPosition,
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

type Number = usize;

struct Rect {
    min_x: Number,
    max_x: Number,
    min_y: Number,
    max_y: Number,
}

impl Rect {
    fn new(p1: Tile, p2: Tile) -> Self {
        Self {
            min_x: p1.x.min(p2.x),
            max_x: p1.x.max(p2.x),
            min_y: p1.y.min(p2.y),
            max_y: p1.y.max(p2.y),
        }
    }

    fn intersects_with(&self, other: &Self) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }

    fn area(&self) -> Number {
        (self.min_x.abs_diff(self.max_x) + 1) * (self.min_y.abs_diff(self.max_y) + 1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    x: Number,
    y: Number,
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.split(",");
        let x = numbers.next().ok_or(ParseError::InvalidPosition)?.parse()?;
        let y = numbers.next().ok_or(ParseError::InvalidPosition)?.parse()?;
        Ok(Self { x, y })
    }
}

pub struct Floor {
    tiles: Vec<Tile>,
}

impl FromStr for Floor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(Tile::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { tiles })
    }
}

impl Floor {
    pub fn find_largest_area(&self, check_intersect: bool) -> Number {
        let mut largest = 0;

        let mut edges: Vec<Rect> = self
            .tiles
            .windows(2)
            .map(|t| Rect::new(t[0], t[1]))
            .collect();

        edges.push(
            self.tiles
                .first()
                .map(|t1| Rect::new(*t1, *self.tiles.last().unwrap()))
                .unwrap(),
        );

        for i in 0..(self.tiles.len() - 1) {
            for j in i..self.tiles.len() {
                let r1 = self.tiles[i];
                let r2 = self.tiles[j];
                let rect = Rect::new(r1, r2);
                let area = rect.area();

                if check_intersect && !check_intersections(&edges, &rect) {
                    continue;
                }

                largest = largest.max(area);
            }
        }

        largest
    }
}

fn check_intersections(edges: &Vec<Rect>, rect: &Rect) -> bool {
    for edge in edges {
        if rect.intersects_with(edge) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn solution_1() {
        let floor = Floor::from_str(EXAMPLE).unwrap();
        assert_eq!(floor.find_largest_area(false), 50)
    }

    #[test]
    fn solution_2() {
        let floor = Floor::from_str(EXAMPLE).unwrap();
        assert_eq!(floor.find_largest_area(true), 24)
    }
}
