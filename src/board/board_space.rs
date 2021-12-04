use super::piece::Piece;

#[derive(Debug)]
pub struct BoardSpace {
    pub board_space_indices: Vec<u32>,
    pub piece: Piece,
}

impl BoardSpace {
    pub fn new(board_space_indices: Vec<u32>, piece: Piece) -> Self {
        Self {
            board_space_indices,
            piece,
        }
    }
}
