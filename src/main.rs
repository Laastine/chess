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

fn main() {
    // 0,0 equals square A1 and white rook,
    let board: [[isize; 8]; 8] = [
        [WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_KING, WHITE_QUEEN, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK],
        [WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN],
        [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
        [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
        [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
        [EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE, EMPTY_SQUARE],
        [BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_KING, BLACK_QUEEN, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK],
        [BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN],
    ];

    dbg!(board);
}
