use super::grid;

pub struct Piece {
    pub grid: u128,
    weight: u32,
}

pub mod Pieces {
    use super::grid::{NB_ROWS, ROW_LENGTH};
    use super::Piece;

    pub const DOT: Piece = Piece {
        grid: 1 << ROW_LENGTH * NB_ROWS - 1,
        weight: 1,
    };
    pub const DASH2: Piece = Piece {
        grid: DOT.grid | DOT.grid >> DOT.weight,
        weight: DOT.weight * 2,
    };
    pub const DASH3: Piece = Piece {
        grid: DOT.grid | DASH2.grid >> DOT.weight,
        weight: DOT.weight + DASH2.weight,
    };
    pub const DASH4: Piece = Piece {
        grid: DASH2.grid + DASH2.grid >> DASH2.weight,
        weight: DASH2.weight * 2,
    };
    pub const DASH5: Piece = Piece {
        grid: DOT.grid | DASH4.grid >> DOT.weight,
        weight: DOT.weight + DASH4.weight,
    };
    pub const BAR2: Piece = Piece {
        grid: DOT.grid | DOT.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight * 2,
    };
    pub const BAR3: Piece = Piece {
        grid: DOT.grid | BAR2.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight + BAR2.weight,
    };
    pub const BAR4: Piece = Piece {
        grid: BAR2.grid | BAR2.grid >> BAR2.weight * ROW_LENGTH,
        weight: BAR2.weight * 2,
    };
    pub const BAR5: Piece = Piece {
        grid: DOT.grid | BAR4.grid >> DOT.weight * ROW_LENGTH,
        weight: DOT.weight + BAR4.weight,
    };
}
