use std::{
    fmt::Display,
    ops::{self, AddAssign},
    sync::Mutex,
};

use super::pieces::Pieces::PIECES;
use itertools::Itertools;
use rayon::prelude::{ParallelBridge, ParallelIterator};

pub const ROW_LENGTH: u16 = 10;
pub const NB_ROWS: u16 = ROW_LENGTH;

pub const EMPTY: Grid = Grid { grid: 0, weight: 0 };
pub const ROW: Grid = Grid {
    grid: (1 << ROW_LENGTH) - 1 << ROW_LENGTH * (NB_ROWS - 1),
    weight: ROW_LENGTH,
};
pub const COL: Grid = Grid {
    grid: 0x8020080200802008020080200,
    weight: NB_ROWS,
};
// (0..NB_ROWS)
//    .fold(EMPTY, |prev, i| prev + Pieces::DOT.moved(i, 0))
//    .grid

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Grid {
    pub grid: u128,
    pub weight: u16,
}

impl Grid {
    pub fn new(grid: u128) -> Grid {
        Grid {
            grid,
            weight: grid.count_ones() as u16,
        }
    }

    pub fn from_str(grid: &str) -> Option<Grid> {
        // From binary string or decimal string
        if let Ok(grid) = u128::from_str_radix(grid, 2).or(u128::from_str_radix(grid, 10)) {
            Some(Grid::new(grid).simplify())
        } else {
            None
        }
    }

    pub fn display(&self, other: &Grid, c: &str) -> String {
        let mut grid_str = String::new();
        grid_str += "  0 1 2 3 4 5 6 7 8 9\n";

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

        grid_str + "^ grid: " + &(self + other).grid.to_string()
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

    pub fn fits(&self, other: &Grid) -> bool {
        self.grid & other.grid == 0
    }

    fn start_non_empty_row(&self) -> Option<Grid> {
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
        if let (Some(rs), Some(rm)) = (self.start_non_empty_row(), moved.start_non_empty_row()) {
            for row in 0..NB_ROWS {
                let row = ROW.moved(row.into(), 0);
                if self.fits(&row) {
                    return Some(moved);
                } else if (&rs & &row).weight != (&rm & &row).weight {
                    return None;
                }
            }
        }
        None
    }

    pub fn ways(&self, other: &Grid) -> Vec<Grid> {
        let mut ways = Vec::new();
        for row in 0..NB_ROWS {
            for col in 0..ROW_LENGTH {
                if let Some(moved) = other.valid_moved(row.into(), col.into()) {
                    if self.fits(&moved) {
                        ways.push(moved);
                    }
                }
            }
        }
        ways
    }

    pub fn contains(&self, other: &Grid) -> bool {
        self.grid & other.grid == other.grid
    }

    pub fn simplify(self) -> Grid {
        let mut to_remove = Vec::new();

        for row in 0..NB_ROWS {
            let row = ROW.moved(row.into(), 0);
            if self.contains(&row) {
                to_remove.push(row);
            }
        }

        for col in 0..ROW_LENGTH {
            let col = COL.moved(0, col.into());
            if self.contains(&col) {
                to_remove.push(col);
            }
        }

        to_remove.iter().fold(self, |prev, m| prev - m)
    }

    pub fn score(&self) -> u32 {
        let mut score: u32 = 0;
        for (piece, prob) in PIECES {
            score += self.ways(&piece).len() as u32 * piece.weight as u32 * prob
        }
        score
    }

    pub fn optimize(&self, pieces: [Grid; 3]) -> Option<([Grid; 3], Grid)> {
        // (ordered moved pieces, result_board)
        let max = Mutex::new((0, None));

        pieces
            .iter()
            .permutations(pieces.len())
            .unique()
            .par_bridge()
            .for_each(|pieces| {
                for way1 in self.ways(&pieces[0]) {
                    let res1 = (self + &way1).simplify();
                    for way2 in res1.ways(&pieces[1]) {
                        let res2 = (&res1 + &way2).simplify();
                        for way3 in res2.ways(&pieces[2]) {
                            let res3 = (&res2 + &way3).simplify();
                            let score = res3.score();
                            let mut max = max.lock().unwrap();
                            if score > max.0 {
                                *max = (score, Some(([way1, way2, way3], res3)));
                            }
                        }
                    }
                }
            });

        return max.lock().unwrap().1;
    }
}

impl ops::Add<&Grid> for &Grid {
    type Output = Grid;

    fn add(self, rhs: &Grid) -> Self::Output {
        Grid::new(self.grid | rhs.grid)
    }
}

impl ops::Add<Grid> for Grid {
    type Output = Grid;

    fn add(self, rhs: Grid) -> Self::Output {
        &self + &rhs
    }
}

impl AddAssign<&Grid> for Grid {
    fn add_assign(&mut self, rhs: &Grid) {
        *self = Grid::new(self.grid + rhs.grid)
    }
}

impl ops::Sub<&Grid> for Grid {
    type Output = Grid;

    fn sub(self, rhs: &Grid) -> Self::Output {
        Grid::new(self.grid & !rhs.grid)
    }
}

impl ops::Sub<Grid> for Grid {
    type Output = Grid;

    fn sub(self, rhs: Grid) -> Self::Output {
        self - &rhs
    }
}

impl ops::BitAnd<&Grid> for &Grid {
    type Output = Grid;

    fn bitand(self, rhs: &Grid) -> Self::Output {
        Grid::new(self.grid & rhs.grid)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(&EMPTY, ""))
    }
}
