mod repr;

use repr::{grid::grid_str, pieces::Pieces};

fn main() {
    println!(
        "{}",
        grid_str(Pieces::BAR5.grid | (Pieces::DASH2.grid >> (3 * 10 + 1)))
    );
}
