use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    ParseTile(char),
}

enum Tile {
    Empty,
    PaperRoll,
}

impl TryFrom<char> for Tile {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::PaperRoll),
            invalid => Err(ParseError::ParseTile(invalid)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    row: usize,
    column: usize,
}

pub struct Row {
    columns: Vec<Tile>,
}

impl FromStr for Row {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row {
            columns: s
                .chars()
                .map(Tile::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub struct Layout {
    rows: Vec<Row>,
}

impl Layout {
    // Returns lengths of (rows, columns)
    #[inline(always)]
    fn get_bounds(&self) -> (usize, usize) {
        (
            self.rows.len(),
            self.rows
                .first()
                .map(|row| row.columns.len())
                .unwrap_or_default(),
        )
    }

    #[inline(always)]
    fn get_tile(&self, coord: &Coordinate) -> Option<&Tile> {
        self.rows
            .get(coord.row)
            .and_then(|row| row.columns.get(coord.column))
    }

    #[inline(always)]
    fn get_tile_mut(&mut self, coord: &Coordinate) -> Option<&mut Tile> {
        self.rows
            .get_mut(coord.row)
            .and_then(|row| row.columns.get_mut(coord.column))
    }

    fn count_adjacent_rolls(&self, coord: &Coordinate) -> i8 {
        let start = coord.row.saturating_sub(1);
        let end = coord.row.saturating_add(1);
        let start_col = coord.column.saturating_sub(1);
        let end_col = if coord.column < self.rows.first().unwrap().columns.len() {
            coord.column + 1
        } else {
            coord.column
        };

        let mut adjacent_rolls = 0;

        for row in start..=end {
            for column in start_col..=end_col {
                let query_coord = Coordinate { row, column };

                if query_coord != *coord
                    && let Some(tile) = self.get_tile(&query_coord)
                    && let Tile::PaperRoll = tile
                {
                    adjacent_rolls += 1
                }
            }
        }

        adjacent_rolls
    }
}

pub struct StorageRoom {
    layout: Layout,
}

impl FromStr for StorageRoom {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let layout = Layout {
            rows: s
                .lines()
                .map(Row::from_str)
                .collect::<Result<Vec<_>, _>>()?,
        };

        Ok(Self { layout })
    }
}

impl StorageRoom {
    pub fn count_accessible_paper_rolls(&self) -> usize {
        let mut count = 0;
        let (rows, columns) = self.layout.get_bounds();

        for row in 0..rows {
            for column in 0..columns {
                let coord = Coordinate { row, column };
                let tile = self.layout.get_tile(&coord);
                if let Some(Tile::PaperRoll) = tile
                    && self.layout.count_adjacent_rolls(&coord) < 4
                {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_accessible_paper_rolls_incrementally(&mut self) -> usize {
        let mut total_count = 0;
        let (rows, columns) = self.layout.get_bounds();

        loop {
            let mut count = 0;

            for row in 0..rows {
                for column in 0..columns {
                    let coord = Coordinate { row, column };
                    if let Some(Tile::PaperRoll) = self.layout.get_tile(&coord)
                        && self.layout.count_adjacent_rolls(&coord) < 4
                    {
                        let tile = self.layout.get_tile_mut(&coord).unwrap();
                        *tile = Tile::Empty;
                        count += 1;
                    }
                }
            }

            total_count += count;

            if count == 0 {
                break;
            }
        }

        total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn solution_1() {
        let storage = StorageRoom::from_str(EXAMPLE).unwrap();
        assert_eq!(storage.count_accessible_paper_rolls(), 13);
    }

    #[test]
    fn solution_2() {
        let mut storage = StorageRoom::from_str(EXAMPLE).unwrap();
        assert_eq!(storage.count_accessible_paper_rolls_incrementally(), 43)
    }
}
