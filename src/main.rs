mod repr;

use repr::pieces::Pieces;

use crate::repr::grid::{Grid, COL, ROW};

fn main() {
    println!("{}", (COL + ROW).simplify());
}
