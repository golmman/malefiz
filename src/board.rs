use self::board_space::BoardSpace;
use self::display::BOARD;
use self::piece::Piece;

pub mod board_space;
pub mod display;
pub mod piece;

#[derive(Debug)]
pub struct Board {
    pub board_spaces: Vec<BoardSpace>,
}

impl Board {
    pub fn new() -> Self {
        let mut board_spaces: Vec<BoardSpace> = Vec::new();

        let mut board_arrays: Vec<Vec<Option<u32>>> = Vec::new();

        // setup two dimensional array of board spaces from BOARD
        let board_lines = BOARD.split("\n");
        for board_line in board_lines {
            let mut board_array: Vec<Option<u32>> = Vec::new();

            let board_line_bytes = board_line.as_bytes();

            for x in 0..17 {
                let i = x * 4 + 2;

                let board_space_index = if i < board_line_bytes.len() {
                    let number_as_bytes = vec![
                        board_line_bytes[i - 2],
                        board_line_bytes[i - 1],
                        board_line_bytes[i - 0],
                    ];

                    let number_as_str = std::str::from_utf8(&number_as_bytes).unwrap();

                    number_as_str.parse::<u32>().ok()
                } else {
                    None
                };

                board_array.push(board_space_index);
            }

            board_arrays.push(board_array);
        }

        // link board spaces with the help of the previously created two dimensional array
        for (row, board_array) in board_arrays.iter().enumerate() {
            for (col, board_space_index) in board_array.iter().enumerate() {
                if let Some(board_space_index) = board_space_index {
                    if board_space_index > &111 {
                        // skip start areas, which are setup further down
                        board_spaces.push(BoardSpace::new(Vec::new(), Piece::NONE));
                        continue;
                    }

                    let mut board_space_indices = Vec::<u32>::new();
                    if row > 0 {
                        if let Some(top_index) = board_arrays[row - 1][col] {
                            board_space_indices.push(top_index);
                        }
                    }
                    if row < board_array.len() - 1 {
                        if let Some(bottom_index) = board_arrays[row + 1][col] {
                            if bottom_index < 112 {
                                // don't link to the start areas
                                board_space_indices.push(bottom_index);
                            }
                        }
                    }
                    if col > 0 {
                        if let Some(left_index) = board_arrays[row][col - 1] {
                            board_space_indices.push(left_index);
                        }
                    }
                    if col < board_array.len() - 1 {
                        if let Some(right_index) = board_arrays[row][col + 1] {
                            board_space_indices.push(right_index);
                        }
                    }

                    board_spaces.push(BoardSpace::new(board_space_indices, Piece::NONE));
                }
            }
        }

        // red start area
        board_spaces[112] = BoardSpace::new(vec![97], Piece::RED);
        board_spaces[113] = BoardSpace::new(vec![97], Piece::RED);
        board_spaces[114] = BoardSpace::new(vec![97], Piece::RED);
        board_spaces[124] = BoardSpace::new(vec![97], Piece::RED);
        board_spaces[125] = BoardSpace::new(vec![97], Piece::RED);

        // green start area
        board_spaces[115] = BoardSpace::new(vec![101], Piece::GREEN);
        board_spaces[116] = BoardSpace::new(vec![101], Piece::GREEN);
        board_spaces[117] = BoardSpace::new(vec![101], Piece::GREEN);
        board_spaces[126] = BoardSpace::new(vec![101], Piece::GREEN);
        board_spaces[127] = BoardSpace::new(vec![101], Piece::GREEN);

        // yellow start area
        board_spaces[118] = BoardSpace::new(vec![105], Piece::YELLOW);
        board_spaces[119] = BoardSpace::new(vec![105], Piece::YELLOW);
        board_spaces[120] = BoardSpace::new(vec![105], Piece::YELLOW);
        board_spaces[128] = BoardSpace::new(vec![105], Piece::YELLOW);
        board_spaces[129] = BoardSpace::new(vec![105], Piece::YELLOW);

        // blue start area
        board_spaces[121] = BoardSpace::new(vec![109], Piece::BLUE);
        board_spaces[122] = BoardSpace::new(vec![109], Piece::BLUE);
        board_spaces[123] = BoardSpace::new(vec![109], Piece::BLUE);
        board_spaces[130] = BoardSpace::new(vec![109], Piece::BLUE);
        board_spaces[131] = BoardSpace::new(vec![109], Piece::BLUE);

        // barricade pieces
        board_spaces[9].piece = Piece::BARRICADE;
        board_spaces[28].piece = Piece::BARRICADE;
        board_spaces[37].piece = Piece::BARRICADE;
        board_spaces[40].piece = Piece::BARRICADE;
        board_spaces[47].piece = Piece::BARRICADE;
        board_spaces[51].piece = Piece::BARRICADE;
        board_spaces[73].piece = Piece::BARRICADE;
        board_spaces[77].piece = Piece::BARRICADE;
        board_spaces[81].piece = Piece::BARRICADE;
        board_spaces[85].piece = Piece::BARRICADE;
        board_spaces[89].piece = Piece::BARRICADE;

        Self { board_spaces }
    }
}
