mod repr;

use itertools::Itertools;
use repr::{grid::Grid, pieces::PIECES_NAME};
use std::io::{stdin, stdout, Write};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input.");

    user_input.trim().to_owned()
}

fn main() {
    let mut board = loop {
        if let Ok(grid) = Grid::from_nb_string(get_input("Enter starting board: ")) {
            break grid;
        }
    };
    println!("Starting board:\n{}", board);

    loop {
        let pieces = loop {
            if let Some((Some(&p1), Some(&p2), Some(&p3))) = get_input("Enter 3 pieces: ")
                .split_whitespace()
                .map(|name| PIECES_NAME.get(name))
                .next_tuple()
            {
                break [p1, p2, p3];
            }
        };

        println!("Computing best solution...");
        if let Some((pieces, _)) = board.optimize(pieces) {
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
