use crate::{Move, Movement};

pub const EMPTY_SQUARE: isize = 0;

pub const WHITE_PAWN: isize = 1;
pub const WHITE_KNIGHT: isize = 2;
pub const WHITE_BISHOP: isize = 3;
pub const WHITE_ROOK: isize = 5;
pub const WHITE_QUEEN: isize = 9;
pub const WHITE_KING: isize = 99;

pub const BLACK_PAWN: isize = -1;
pub const BLACK_KNIGHT: isize = -2;
pub const BLACK_BISHOP: isize = -3;
pub const BLACK_ROOK: isize = -5;
pub const BLACK_QUEEN: isize = -9;
pub const BLACK_KING: isize = -99;

// 0,0 equals square A1 and white rook,
pub struct Board {
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

    fn movement_coord_to_board_coord(square: Move) -> Move {
        let col = Board::get_rev_index(square.0);
        let row = Board::get_rev_index(square.1 - 1);
        (row, col)
    }

    #[allow(dead_code)]
    pub fn get_piece_by_movement(&self, square: Move) -> isize {
        let (row, col) = Board::movement_coord_to_board_coord(square);
        self.board[row as usize][col as usize]
    }

    fn get_rev_index(idx: isize) -> isize {
        (7 - idx).abs()
    }

    pub fn update_board(&mut self, movement: &Movement) {
        let board_from = Board::movement_coord_to_board_coord(movement.from);
        let board_to = Board::movement_coord_to_board_coord(movement.to);
        let piece = Clone::clone(&self.board[board_from.0 as usize][board_from.1 as usize]);
        self.board[board_from.0 as usize][board_from.1 as usize] = 0;
        self.board[board_to.0 as usize][board_to.1 as usize] = piece;
    }

    pub fn render(&self) {
        for (idx, y) in self.board.iter().enumerate() {
            print!(" {: >2}", 8-idx);
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
