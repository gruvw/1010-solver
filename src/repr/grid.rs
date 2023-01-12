use super::pieces::PIECES;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::{
    fmt::{Display, Write},
    num::ParseIntError,
    ops,
    str::FromStr,
    sync::Mutex,
};

pub const ROW_LENGTH: u16 = 10;
pub const NB_ROWS: u16 = ROW_LENGTH;

pub const EMPTY: Grid = Grid { grid: 0, weight: 0 };
pub const ROW: Grid = Grid {
    grid: ((1 << ROW_LENGTH) - 1) << (ROW_LENGTH * (NB_ROWS - 1)),
    weight: ROW_LENGTH,
};
pub const COL: Grid = Grid {
    grid: 0x8020080200802008020080200,
    weight: NB_ROWS,
};
// (0..NB_ROWS).fold(EMPTY, |prev, i| &prev + &DOT.moved(i as i32, 0))

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Grid {
    pub grid: u128,
    pub weight: u16,
}

impl Grid {
    pub const fn new(grid: u128) -> Grid {
        Grid {
            grid,
            weight: grid.count_ones() as u16,
        }
    }

    pub fn from_nb_string(grid: String) -> Result<Grid, ParseIntError> {
        // From binary string or decimal string
        u128::from_str_radix(&grid, 2)
            .or_else(|_| grid.parse::<u128>())
            .map(|grid| Grid::new(grid).simplify())
    }

    pub fn simplify(self) -> Grid {
        (0..NB_ROWS)
            .map(|row| ROW.moved(row.into(), 0))
            .chain((0..ROW_LENGTH).map(|col| COL.moved(0, col.into())))
            .filter(|g| self.contains(g))
            .fold(self, |prev, m| prev - &m)
    }

    pub fn display(&self, other: &Grid, c: &str) -> String {
        let mut grid_str = String::from_str("  0 1 2 3 4 5 6 7 8 9\n").unwrap();

        for row in (0..NB_ROWS).rev() {
            grid_str += &((NB_ROWS - row - 1).to_string() + " ");
            for col in (0..ROW_LENGTH).rev() {
                let (row, col) = (row.into(), col.into());
                grid_str += match self.moved(row, col).grid & 1 {
                    1 => "X ",
                    _ => match other.moved(row, col).grid & 1 {
                        1 => c,
                        _ => "  ",
                    },
                };
            }
            grid_str += "\n";
        }
        let total = self + other;
        writeln!(grid_str, "├╴ score: {}", total.simplify().score()).unwrap();
        write!(grid_str, "╰╴ grid: {}", total.grid).unwrap();
        grid_str
    }

    pub const fn contains(&self, other: &Grid) -> bool {
        self.grid & other.grid == other.grid
    }

    pub const fn fits(&self, other: &Grid) -> bool {
        self.grid & other.grid == 0
    }

    pub const fn moved(&self, row: i32, col: i32) -> Grid {
        let offset = row * ROW_LENGTH as i32 + col;
        let grid = if row >= 0 {
            self.grid >> offset
        } else {
            self.grid << -offset
        };
        Grid {
            grid,
            weight: grid.count_ones() as u16,
        }
    }

    // Helpers

    fn shift_empty_rows(&self) -> Option<Grid> {
        for row in 0..NB_ROWS {
            let row_grid = ROW.moved(row.into(), 0);
            if !self.fits(&row_grid) {
                return Some(self.moved(-(row as i32), 0));
            }
        }
        None
    }

    fn valid_moved(&self, row: i32, col: i32) -> Option<Grid> {
        let moved = self.moved(row, col);
        if self.weight != moved.weight {
            return None;
        }
        if let (Some(rs), Some(rm)) = (self.shift_empty_rows(), moved.shift_empty_rows()) {
            for row in 0..NB_ROWS {
                let row = ROW.moved(row.into(), 0);
                if self.fits(&row) {
                    // Back on empty row
                    return Some(moved);
                } else if (&rs & &row).weight != (&rm & &row).weight {
                    // Moved has different weight repartition (line wrap occurred)
                    return None;
                }
            }
        }
        None
    }

    fn ways<'a>(&'a self, other: &'a Grid) -> impl Iterator<Item = Grid> + '_ {
        (0..NB_ROWS)
            .flat_map(move |row| {
                (0..ROW_LENGTH).filter_map(move |col| other.valid_moved(row.into(), col.into()))
            })
            .filter(|m| self.fits(m))
    }

    // Decision making

    pub fn score(&self) -> u32 {
        PIECES
            .iter()
            .map(|(piece, prob)| self.ways(piece).count() as u32 * piece.weight as u32 * prob)
            .sum()
    }

    pub fn optimize<'a>(&'a self, pieces: &[Grid]) -> Option<(Vec<Grid>, Grid)> {
        if pieces.is_empty() {
            return Some((pieces.to_vec(), *self));
        }

        // (max score, Option<(ordered moved pieces, result_board)>)
        let max = Mutex::new((0, None));

        (0..pieces.len()).par_bridge().for_each(|i| {
            let tail = [&pieces[..i], &pieces[i + 1..]].concat();
            for way in self.ways(&pieces[i]) {
                if let Some((pieces, res)) = (self + &way).simplify().optimize(&tail) {
                    let score = res.score();
                    let mut max = max.lock().unwrap();
                    if score > max.0 {
                        let mut pieces = pieces.clone();
                        pieces.insert(0, way);
                        *max = (score, Some((pieces, res)));
                    }
                }
            }
        });

        let value = max.lock().unwrap().clone().1;
        value
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(&EMPTY, ""))
    }
}

// Inner Grid operations

impl ops::Add<&Grid> for &Grid {
    type Output = Grid;

    fn add(self, rhs: &Grid) -> Self::Output {
        Grid::new(self.grid | rhs.grid)
    }
}

impl ops::Sub<&Grid> for Grid {
    type Output = Grid;

    fn sub(self, rhs: &Grid) -> Self::Output {
        Grid::new(self.grid & !rhs.grid)
    }
}

impl ops::BitAnd<&Grid> for &Grid {
    type Output = Grid;

    fn bitand(self, rhs: &Grid) -> Self::Output {
        Grid::new(self.grid & rhs.grid)
    }
}
