use self::Pieces::*;
use super::grid::{self, Grid};
use phf::{phf_map, Map};

// (piece, relative probability)
pub const PIECES: [(Grid, u32); 19] = [
    (DOT, 2),
    (DASH_2, 3),
    (DASH_3, 3),
    (DASH_4, 2),
    (DASH_5, 2),
    (BAR_2, 3),
    (BAR_3, 3),
    (BAR_4, 2),
    (BAR_5, 2),
    (DOUBLE_N, 2),
    (DOUBLE_E, 2),
    (DOUBLE_S, 2),
    (DOUBLE_W, 2),
    (TRIPLE_N, 1),
    (TRIPLE_E, 1),
    (TRIPLE_S, 1),
    (TRIPLE_W, 1),
    (SQUARE_SMALL, 6),
    (SQUARE_BIG, 2),
];

pub static PIECES_NAME: Map<&'static str, Grid> = phf_map!(
    "d" => DOT,
    "d2" => DASH_2,
    "d3" => DASH_3,
    "d4" => DASH_4,
    "d5" => DASH_5,
    "b2" => BAR_2,
    "b3" => BAR_3,
    "b4" => BAR_4,
    "b5" => BAR_5,
    "dn" => DOUBLE_N,
    "de" => DOUBLE_E,
    "ds" => DOUBLE_S,
    "dw" => DOUBLE_W,
    "tn" => TRIPLE_N,
    "te" => TRIPLE_E,
    "ts" => TRIPLE_S,
    "tw" => TRIPLE_W,
    "ss" => SQUARE_SMALL,
    "sb" => SQUARE_BIG,
);

// "Enum like" for pieces
#[allow(non_snake_case)]
pub mod Pieces {
    use super::grid::{Grid, NB_ROWS, ROW_LENGTH};

    pub const DOT: Grid = Grid {
        grid: 1 << (ROW_LENGTH * NB_ROWS - 1),
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
        grid: DOT.grid | (DOT.grid >> (DOT.weight * ROW_LENGTH)),
        weight: DOT.weight * 2,
    };
    pub const BAR_3: Grid = Grid {
        grid: DOT.grid | (BAR_2.grid >> (DOT.weight * ROW_LENGTH)),
        weight: DOT.weight + BAR_2.weight,
    };
    pub const BAR_4: Grid = Grid {
        grid: BAR_2.grid | (BAR_2.grid >> (BAR_2.weight * ROW_LENGTH)),
        weight: BAR_2.weight * 2,
    };
    pub const BAR_5: Grid = Grid {
        grid: DOT.grid | (BAR_4.grid >> (DOT.weight * ROW_LENGTH)),
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
        grid: BAR_2.grid | (DOT.grid >> (ROW_LENGTH + DOT.weight)),
        weight: BAR_2.weight + DOT.weight,
    };
    pub const TRIPLE_N: Grid = Grid {
        grid: DASH_3.grid | BAR_2.grid >> ROW_LENGTH,
        weight: DASH_3.weight + BAR_2.weight,
    };
    pub const TRIPLE_E: Grid = Grid {
        grid: DASH_3.grid | BAR_2.grid >> (ROW_LENGTH + DASH_2.weight),
        weight: DASH_3.weight + BAR_2.weight,
    };
    pub const TRIPLE_S: Grid = Grid {
        grid: BAR_3.grid >> DASH_2.weight | DASH_2.grid >> (ROW_LENGTH * DASH_2.weight),
        weight: BAR_3.weight + DASH_2.weight,
    };
    pub const TRIPLE_W: Grid = Grid {
        grid: BAR_3.grid | DASH_2.grid >> (ROW_LENGTH * DASH_2.weight + DOT.weight),
        weight: BAR_3.weight + DASH_2.weight,
    };
    pub const SQUARE_SMALL: Grid = Grid {
        grid: BAR_2.grid | BAR_2.grid >> DOT.weight,
        weight: BAR_2.weight * 2,
    };
    pub const SQUARE_BIG: Grid = Grid {
        grid: BAR_3.grid | BAR_3.grid >> DOT.weight | BAR_3.grid >> (DOT.weight * 2),
        weight: BAR_3.weight * 3,
    };
}
