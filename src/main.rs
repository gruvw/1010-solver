mod repr;

use std::io::{stdin, stdout, Write};

use crate::repr::{grid::Grid, pieces::Pieces::*};

fn main() {
    let mut board = loop {
        print!("Enter starting board: ");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        if let Some(grid) = Grid::from_str(&user_input.trim()) {
            break grid;
        }
    };
    println!("Initial board:\n{}", board);

    loop {
        // TODO ask for pieces (loop)
        if let Some((pieces, _)) = board.optimize([DASH_4, DOT, TRIPLE_E]) {
            for (i, piece) in pieces.iter().enumerate() {
                println!("Piece number {}:\n{}", i + 1, board.display(piece, "O "));
                board = (&board + piece).simplify();
            }
            println!("Result:\n{}\n", board);
        } else {
            return println!("Game lost!");
        }
    }
}
