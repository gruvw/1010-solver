use super::grid;

pub mod Pieces {
    use super::grid::{Grid, NB_ROWS, ROW_LENGTH};

    pub const DOT: Grid = Grid {
        grid: 1 << ROW_LENGTH * NB_ROWS - 1,
        weight: 1,
    };
    pub const DASH2: Grid = Grid {
        grid: DOT.grid | DOT.grid >> DOT.weight,
        weight: DOT.weight * 2,
    };
    pub const DASH3: Grid = Grid {
        grid: DOT.grid | DASH2.grid >> DOT.weight,
        weight: DOT.weight + DASH2.weight,
    };
    pub const DASH4: Grid = Grid {
        grid: DASH2.grid | DASH2.grid >> DASH2.weight,
        weight: DASH2.weight * 2,
    };
    pub const DASH5: Grid = Grid {
        grid: DOT.grid | DASH4.grid >> DOT.weight,
        weight: DOT.weight + DASH4.weight,
    };
    pub const BAR2: Grid = Grid {
        grid: DOT.grid | DOT.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight * 2,
    };
    pub const BAR3: Grid = Grid {
        grid: DOT.grid | BAR2.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight + BAR2.weight,
    };
    pub const BAR4: Grid = Grid {
        grid: BAR2.grid | BAR2.grid >> BAR2.weight * ROW_LENGTH,
        weight: BAR2.weight * 2,
    };
    pub const BAR5: Grid = Grid {
        grid: DOT.grid | BAR4.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight + BAR4.weight,
    };
    // pub const L: Grid = BAR2 + DOT.moved(0, 1);
}
