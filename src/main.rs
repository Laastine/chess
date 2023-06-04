mod board;
mod movement;
mod logic;

use crate::board::{Board};
use crate::movement::Movement;

pub type Move = (isize, isize);
pub type Pos = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum MoveError {
    IllegalMovementNotation,
    FromSquareOutOfBoard,
    ToSquareOutOfBoard,
}

fn square_to_index(square: &str) -> Move {
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

    (row, column as isize)
}

#[cfg(test)]
mod test {
    use crate::{board::Board, board::EMPTY_SQUARE, MoveError, board::WHITE_KNIGHT, board::WHITE_PAWN};
    use crate::movement::Movement;

    #[test]
    fn pawn_movement_notation() {
        let mov = Movement::parse_movement_notation("e2 e4").unwrap();
        assert_eq!(mov.from, (4, 2));
        assert_eq!(mov.to, (4, 4));
    }

    #[test]
    fn knight_movement_notation() {
        let mov = Movement::parse_movement_notation("b1 c3").unwrap();
        assert_eq!(mov.from, (1, 1));
        assert_eq!(mov.to, (2, 3));
    }

    #[test]
    fn pawn_movement_to_board_square_index() {
        let mut board = Board::new();
        let mov = Movement::parse_movement_notation("e2 e4").unwrap();

        assert_eq!(board.get_piece_by_movement(mov.from), WHITE_PAWN);
        assert_eq!(board.get_piece_by_movement(mov.to), EMPTY_SQUARE);
        board.update_board(&mov);
        assert_eq!(board.get_piece_by_movement(mov.from), EMPTY_SQUARE);
        assert_eq!(board.get_piece_by_movement(mov.to), WHITE_PAWN);
    }

    #[test]
    fn knight_movement_to_board_square_index() {
        let mut board = Board::new();
        let mov = Movement::parse_movement_notation("b1 c3").unwrap();

        assert_eq!(board.get_piece_by_movement(mov.from), WHITE_KNIGHT);
        assert_eq!(board.get_piece_by_movement(mov.to), EMPTY_SQUARE);
        board.update_board(&mov);
        assert_eq!(board.get_piece_by_movement(mov.from), EMPTY_SQUARE);
        assert_eq!(board.get_piece_by_movement(mov.to), WHITE_KNIGHT);
    }

    #[test]
    fn check_legal_movement_notation() {
        let mov = Movement::parse_movement_notation("b1c1");
        assert_eq!(mov, Err(MoveError::IllegalMovementNotation))
    }

    #[test]
    fn check_out_of_bounds_movement() {
        let mov = Movement::parse_movement_notation("b1 c11");
        assert_eq!(mov, Err(MoveError::ToSquareOutOfBoard))
    }
}


fn main() {
    let mut board = Board::new();

    board.update_board(&Movement::parse_movement_notation("e2 e4").unwrap());
    board.update_board(&Movement::parse_movement_notation("e7 e5").unwrap());
    board.update_board(&Movement::parse_movement_notation("b1 c3").unwrap());
    board.update_board(&Movement::parse_movement_notation("d7 d6").unwrap());
    board.update_board(&Movement::parse_movement_notation("f1 c4").unwrap());

    board.render();
}
