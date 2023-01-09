use std::{fmt::Display, ops};

pub const ROW_LENGTH: u32 = 10;
pub const NB_ROWS: u32 = ROW_LENGTH;

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

pub struct Grid {
    pub grid: u128,
    pub weight: u32,
}

impl Grid {
    pub const fn moved(&self, row: u32, col: u32) -> Grid {
        let grid = self.grid >> (row * ROW_LENGTH + col);
        Grid {
            grid,
            weight: grid.count_ones(),
        }
    }

    pub fn fits(&self, other: &Grid) -> bool {
        self.grid & other.grid == 0
    }

    pub fn contains(&self, other: &Grid) -> bool {
        self.grid & other.grid == other.grid
    }

    pub fn simplify(self) -> Grid {
        let mut to_remove = Vec::new();

        for r in 0..NB_ROWS {
            let row = ROW.moved(r, 0);
            if self.contains(&row) {
                to_remove.push(row);
            }
        }

        for c in 0..ROW_LENGTH {
            let col = COL.moved(0, c);
            if self.contains(&col) {
                to_remove.push(col);
            }
        }

        to_remove.iter().fold(self, |prev, m| prev - m)
    }
}

impl ops::Add<Grid> for Grid {
    type Output = Grid;

    fn add(self, rhs: Grid) -> Self::Output {
        let grid = self.grid | rhs.grid;
        Grid {
            grid,
            weight: grid.count_ones(),
        }
    }
}

impl ops::Sub<&Grid> for Grid {
    type Output = Grid;

    fn sub(self, rhs: &Grid) -> Self::Output {
        let grid = self.grid & !rhs.grid;
        Grid {
            grid,
            weight: grid.count_ones(),
        }
    }
}

impl ops::Sub<Grid> for Grid {
    type Output = Grid;

    fn sub(self, rhs: Grid) -> Self::Output {
        self - &rhs
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_str = String::new();

        for i in (0..NB_ROWS).rev() {
            for j in (0..ROW_LENGTH).rev() {
                grid_str += match self.grid >> (i * ROW_LENGTH + j) & 1 {
                    1 => "X ",
                    _ => "  ",
                };
            }
            grid_str += "\n";
        }

        write!(f, "{}", grid_str)
    }
}
