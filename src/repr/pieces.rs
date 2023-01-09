use super::grid;

pub mod Pieces {
    use super::grid::{Grid, NB_ROWS, ROW_LENGTH};

    pub const DOT: Grid = Grid {
        grid: 1 << ROW_LENGTH * NB_ROWS - 1,
        weight: 1,
    };
    pub const DASH_2: Grid = Grid {
        grid: DOT.grid | DOT.grid >> DOT.weight,
        weight: DOT.weight * 2,
    };
    pub const DASH_3: Grid = Grid {
        grid: DOT.grid | DASH_2.grid >> DOT.weight,
        weight: DOT.weight + DASH_2.weight,
    };
    pub const DASH_4: Grid = Grid {
        grid: DASH_2.grid | DASH_2.grid >> DASH_2.weight,
        weight: DASH_2.weight * 2,
    };
    pub const DASH_5: Grid = Grid {
        grid: DOT.grid | DASH_4.grid >> DOT.weight,
        weight: DOT.weight + DASH_4.weight,
    };
    pub const BAR_2: Grid = Grid {
        grid: DOT.grid | DOT.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight * 2,
    };
    pub const BAR_3: Grid = Grid {
        grid: DOT.grid | BAR_2.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight + BAR_2.weight,
    };
    pub const BAR_4: Grid = Grid {
        grid: BAR_2.grid | BAR_2.grid >> BAR_2.weight * ROW_LENGTH,
        weight: BAR_2.weight * 2,
    };
    pub const BAR_5: Grid = Grid {
        grid: DOT.grid | BAR_4.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight + BAR_4.weight,
    };
    pub const DOUBLE_N: Grid = Grid {
        grid: DASH_2.grid | DOT.grid >> ROW_LENGTH,
        weight: DASH_2.weight + DOT.weight,
    };
    pub const DOUBLE_E: Grid = Grid {
        grid: DOT.grid | BAR_2.grid >> DOT.weight,
        weight: DOT.weight + BAR_2.weight,
    };
    pub const DOUBLE_S: Grid = Grid {
        grid: DOT.grid >> DOT.weight | DASH_2.grid >> ROW_LENGTH,
        weight: DOT.weight + DASH_2.weight,
    };
    pub const DOUBLE_W: Grid = Grid {
        grid: BAR_2.grid | DOT.grid >> ROW_LENGTH + DOT.weight,
        weight: BAR_2.weight + DOT.weight,
    };
    pub const TRIPLE_N: Grid = Grid {
        grid: DASH_3.grid | BAR_2.grid >> ROW_LENGTH,
        weight: DASH_3.weight + BAR_2.weight,
    };
    pub const TRIPLE_E: Grid = Grid {
        grid: DASH_3.grid | BAR_2.grid >> ROW_LENGTH + DASH_2.weight,
        weight: DASH_3.weight + BAR_2.weight,
    };
    pub const TRIPLE_S: Grid = Grid {
        grid: BAR_3.grid >> DASH_2.weight | DASH_2.grid >> ROW_LENGTH * DASH_2.weight,
        weight: BAR_3.weight + DASH_2.weight,
    };
    pub const TRIPLE_W: Grid = Grid {
        grid: BAR_3.grid | DASH_2.grid >> ROW_LENGTH * DASH_2.weight + DOT.weight,
        weight: BAR_3.weight + DASH_2.weight,
    };
    pub const SQUARE_SMALL: Grid = Grid {
        grid: BAR_2.grid | BAR_2.grid >> DOT.weight,
        weight: BAR_2.weight * 2,
    };
    pub const SQUARE_BIG: Grid = Grid {
        grid: BAR_3.grid | BAR_3.grid >> DOT.weight | BAR_3.grid >> DOT.weight * 2,
        weight: BAR_3.weight * 3,
    };
}
