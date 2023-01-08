pub const ROW_LENGTH: u32 = 10;
pub const NB_ROWS: u32 = ROW_LENGTH;

// const FULL_ROW: u128 =;

pub fn grid_str(grid: u128) -> String {
    let mut grid_str = String::new();

    for i in (0..NB_ROWS).rev() {
        for j in (0..ROW_LENGTH).rev() {
            grid_str += match grid >> (i * ROW_LENGTH + j) & 1 {
                1 => "█",
                _ => " ",
            };
        }
        grid_str += "\n";
    }

    grid_str
}
