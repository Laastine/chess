const EMPTY_SQUARE: isize = 0;

const WHITE_PAWN: isize = 1;
const WHITE_KNIGHT: isize = 2;
const WHITE_BISHOP: isize = 3;
const WHITE_ROOK: isize = 5;
const WHITE_QUEEN: isize = 9;
const WHITE_KING: isize = std::isize::MAX;

const BLACK_PAWN: isize = -1;
const BLACK_KNIGHT: isize = -2;
const BLACK_BISHOP: isize = -3;
const BLACK_ROOK: isize = -5;
const BLACK_QUEEN: isize = -9;
const BLACK_KING: isize = -std::isize::MAX;


struct Movement {
    from: (usize, usize),
    to: (usize, usize),
    capture: bool,
    castle: bool,
    en_passant: bool,
}

impl Movement {
    pub fn parse_movement_notation(&self, movement: &str) -> Movement {
        let parts: Vec<&str> = movement.split_ascii_whitespace().collect();
        let (from_notation, to_notation): (&str, &str) = if parts.len() != 2 {
            panic!("Illegal move notation")
        } else {
            (parts[0], parts[1])
        };

        let from = self.square_to_index(from_notation);
        let to = self.square_to_index(to_notation);

        Movement {
            from,
            to,
            capture: false,
            castle: false,
            en_passant: false,
        }
    }

    fn square_to_index(&self, square: &str) -> (usize, usize) {
        let mut chrs = square.chars();
        let foo: (char, &str) = match chrs.next() {
            Some(c) => (c, chrs.as_str()),
            None => panic!("Can not convert square to")
        };
        let row = self.row_to_index(foo.0);

        let column: usize = match foo.1.parse::<usize>() {
            Ok(c) => c,
            Err(e) => panic!("Illegal column notation {}", e),
        };

        (row, column)
    }

    fn row_to_index(&self, chr: char) -> usize {
        match chr {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Illegal row notation"),
        }
    }
}

// 0,0 equals square A1 and white rook,
struct Board {
    board: [[isize; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [
                [WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_KING, WHITE_QUEEN, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK],
                [WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_KING, BLACK_QUEEN, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK],
                [BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN],
            ]
        }
    }

    fn movement_to_board_state(&mut self, movement: &str) -> [[isize; 8]; 8] {

        self.board
    }
}

fn main() {
    let board = Board::new();

    dbg!(board.board);
}
