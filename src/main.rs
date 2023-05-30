const EMPTY_SQUARE: isize = 0;

const WHITE_PAWN: isize = 1;
const WHITE_KNIGHT: isize = 2;
const WHITE_BISHOP: isize = 3;
const WHITE_ROOK: isize = 5;
const WHITE_QUEEN: isize = 9;
const WHITE_KING: isize = 99;

const BLACK_PAWN: isize = -1;
const BLACK_KNIGHT: isize = -2;
const BLACK_BISHOP: isize = -3;
const BLACK_ROOK: isize = -5;
const BLACK_QUEEN: isize = -9;
const BLACK_KING: isize = -99;

#[derive(PartialEq, Debug)]
struct Movement {
    from: (usize, usize),
    to: (usize, usize),
    capture: bool,
    castle: bool,
    en_passant: bool,
}

fn square_to_index(square: &str) -> (usize, usize) {
    let mut chrs = square.chars();
    let sq: (char, &str) = match chrs.next() {
        Some(c) => (c, chrs.as_str()),
        None => panic!("Can not convert square to")
    };
    let row =
        match sq.0 {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Illegal row notation"),
        };

    let column: usize = match sq.1.parse::<usize>() {
        Ok(c) => c,
        Err(e) => panic!("Illegal column notation {}", e),
    };

    (row, column)
}

impl Movement {
    pub fn parse_movement_notation(movement: &str) -> Movement {
        let parts: Vec<&str> = movement.split_ascii_whitespace().collect();
        let (from_notation, to_notation): (&str, &str) = if parts.len() != 2 {
            panic!("Illegal move notation")
        } else {
            (parts[0], parts[1])
        };

        let from = square_to_index(from_notation);
        let to = square_to_index(to_notation);

        Movement {
            from,
            to,
            capture: false,
            castle: false,
            en_passant: false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{BLACK_QUEEN, BLACK_ROOK, Board, EMPTY_SQUARE, Movement, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK};

    #[test]
    fn pawn_movement_notation() {
        let mov = Movement::parse_movement_notation("e2 e4");
        assert_eq!(mov.from, (4, 2));
        assert_eq!(mov.to, (4, 4));
        assert_eq!(mov.en_passant, false);
        assert_eq!(mov.castle, false);
    }

    #[test]
    fn knight_movement_notation() {
        let mov = Movement::parse_movement_notation("b1 c3");
        assert_eq!(mov.from, (1, 1));
        assert_eq!(mov.to, (2, 3));
        assert_eq!(mov.en_passant, false);
        assert_eq!(mov.castle, false);
    }

    #[test]
    fn pawn_movement_to_board_square_index() {
        let mut board = Board::new();
        let mov = Movement::parse_movement_notation("e2 e4");

        assert_eq!(board.get_piece_by_movement(mov.from), WHITE_PAWN);
        assert_eq!(board.get_piece_by_movement(mov.to), EMPTY_SQUARE);
        board.update_board(&mov);
        assert_eq!(board.get_piece_by_movement(mov.from), EMPTY_SQUARE);
        assert_eq!(board.get_piece_by_movement(mov.to), WHITE_PAWN);
    }

    #[test]
    fn knight_movement_to_board_square_index() {
        let mut board = Board::new();
        let mov = Movement::parse_movement_notation("b1 c3");

        dbg!(mov.from.0,mov.from.1);
        dbg!(mov.to.0,mov.to.1);
        assert_eq!(board.get_piece_by_movement(mov.from), WHITE_KNIGHT);
        assert_eq!(board.get_piece_by_movement(mov.to), EMPTY_SQUARE);
        board.update_board(&mov);
        assert_eq!(board.get_piece_by_movement(mov.from), EMPTY_SQUARE);
        assert_eq!(board.get_piece_by_movement(mov.to), WHITE_KNIGHT);
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
                [BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK],
                [BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
                [WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN],
                [WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK],
            ]
        }
    }

    fn movement_coord_to_board_coord(square: (usize, usize)) -> (usize, usize) {
        let col = Board::get_rev_index(square.0);
        let row = Board::get_rev_index(square.1 - 1);
        (row, col)
    }

    pub fn get_piece_by_movement(&self, square: (usize, usize)) -> isize {
        let (row, col) = Board::movement_coord_to_board_coord(square);
        self.board[row][col]
    }


    fn get_rev_index(idx: usize) -> usize {
        (7 - idx as isize).abs().try_into().unwrap()
    }

    pub fn update_board(&mut self, movement: &Movement) {
        let board_from = Board::movement_coord_to_board_coord(movement.from);
        let board_to = Board::movement_coord_to_board_coord(movement.to);
        let piece = Clone::clone(&self.board[board_from.0][board_from.1]);
        self.board[board_from.0][board_from.1] = 0;
        self.board[board_to.0][board_to.1] = piece;
    }

    pub fn render(&self) {
        for (idx, y) in self.board.iter().enumerate() {
            print!(" {: >2}", idx + 1);
            for x in y.iter().rev() {
                print!("{: >5}", x)
            }
            println!()
        }
        println!();
        print!("   ");
        for x in ["A", "B", "C", "D", "E", "F", "G", "H"].iter() {
            print!("{: >5}", x)
        }
        println!()
    }
}

fn main() {
    let mut board = Board::new();

    board.update_board(&Movement::parse_movement_notation("e2 e4"));
    board.update_board(&Movement::parse_movement_notation("e7 e5"));
    board.update_board(&Movement::parse_movement_notation("b1 c3"));
    board.update_board(&Movement::parse_movement_notation("d7 d6"));
    board.update_board(&Movement::parse_movement_notation("f1 c4"));

    board.render();
}
