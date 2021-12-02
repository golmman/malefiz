enum Piece {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    BARRICADE,
}

struct BoardSpace {
    pub board_space_indices: Vec<u32>,
    pub piece: Option<Piece>,
}

impl BoardSpace {
    fn new(board_space_indices: Vec<u32>, piece: Option<Piece>) -> Self {
        Self {
            board_space_indices,
            piece,
        }
    }
}

struct Board {
    pub board_spaces: Vec<BoardSpace>,
}

const BOARD: &str = r"
                                000
001 002 003 004 005 006 007 008 009 010 011 012 013 014 015 016 017
018                                                             019
020 021 022 023 024 025 026 027 028 029 030 031 032 033 034 035 036
                                037
                        038 039 040 041 042
                        043             044
                045 046 047 048 049 050 051 052 053
                054                             055
        056 057 058 059 060 061 062 063 064 065 066 067 068
        069             070             071             072
073 074 075 076 077 078 079 080 081 082 083 084 085 086 087 088 089
090             091             092             093             094
095 096 097 098 099 100 101 102 103 104 105 106 107 108 109 110 111
    112 113 114     115 116 117     118 119 120     121 122 123
    124     125     126     127     128     129     130     131
";

impl Board {
    fn new() -> Self {
        Self {
            board_spaces: vec![
                BoardSpace::new(Vec::new(), None),
                BoardSpace::new(Vec::new(), None),
            ],
        }
    }
}

fn main() {
    println!("Hello, world!");

    let board_lines = BOARD.split("\n");
    for board_line in board_lines {
        println!("{}", board_line);

        let board_line_bytes = board_line.as_bytes();

        for x in 0..16 {
            let i = x * 4 + 2;

            if i < board_line_bytes.len() {
                let number_as_bytes = vec![
                    board_line_bytes[i - 2],
                    board_line_bytes[i - 1],
                    board_line_bytes[i - 0],
                ];

                println!("{:?}", std::str::from_utf8(&number_as_bytes));
            }
        }
    }
}
