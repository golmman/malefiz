use crate::board::Board;

pub mod board;

fn main() {
    let board = Board::new();

    println!("{:?}", board.board_spaces[119]);

    println!("{}", board);
}
