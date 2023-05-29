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
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
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
    use crate::Movement;

    #[test]
    fn test_pawn_movement_notation() {
        let mov = Movement::parse_movement_notation("e2 e4");
        assert_eq!(mov.from, (5, 2));
        assert_eq!(mov.to, (5, 4));
        assert_eq!(mov.en_passant, false);
        assert_eq!(mov.castle, false);
    }

    #[test]
    fn test_knight_movement_notation() {
        let mov = Movement::parse_movement_notation("b1 c3");
        assert_eq!(mov.from, (2, 1));
        assert_eq!(mov.to, (3, 3));
        assert_eq!(mov.en_passant, false);
        assert_eq!(mov.castle, false);
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
}

fn main() {
    let board = Board::new();

    dbg!(board.board);
}
