use crate::{Move, Pos};
use crate::logic::Color::{Black, White};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black = -1,
    White = 1,
}

const LINE_AND_ROW_MOVES: [Move; 28] = [
    (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
    (0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7),
    (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
    (-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0),
];

const DIAGONAL_MOVES: [Move; 28] = [
    (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7),
    (1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7),
    (-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7),
    (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7)];

pub trait Piece {
    fn evaluate(_color: Color, _pos: Pos) -> Vec<Move>;
    fn moves(color: Color, pos: Pos) -> Vec<Move>;
}

struct Pawn {}

impl Piece for Pawn {
    fn evaluate(_color: Color, _pos: Pos) -> Vec<Move> {
        todo!()
    }

    fn moves(color: Color, pos: Pos) -> Vec<Move> {
        if pos.1 == 7 && color == White  {
            let color: isize = 1;
            let ccolor = Clone::clone(&color);
            return vec![(0_isize, pos.1 as isize + color), (0_isize, pos.1 as isize + 2 * ccolor)]
        } else if pos.1 == 2 && color == Black {
          let color: isize = -1;
          let ccolor = Clone::clone(&color);
          return vec![(0_isize, pos.1 as isize + color), (0_isize, pos.1 as isize + 2 * ccolor)]
        }
        vec![(0, 1)]
    }
}

struct Knight {}

impl Piece for Knight {
    fn evaluate(_color: Color, _pos: Pos) -> Vec<Move> {
        vec![]
    }

    fn moves(_color: Color, _pos: Pos) -> Vec<Move> {
        vec![(1, 2), (-1, 2), (1, -2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]
    }
}

struct Bishop {}

impl Piece for Bishop {
    fn evaluate(_color: Color, _pos: Pos) -> Vec<Move> {
        vec![]
    }

    fn moves(_color: Color, _pos: Pos) -> Vec<Move> {
        DIAGONAL_MOVES.to_vec()
    }
}

struct Rook {}

impl Piece for Rook {
    fn evaluate(_color: Color, _pos: Pos) -> Vec<Move> {
        vec![]
    }

    fn moves(_color: Color, _pos: Pos) -> Vec<Move> {
        LINE_AND_ROW_MOVES.to_vec()
    }
}

struct Queen {}

impl Piece for Queen {
    fn evaluate(_color: Color, _pos: Pos) -> Vec<Move> {
        vec![]
    }

    fn moves(_color: Color, _pos: Pos) -> Vec<Move> {
        [DIAGONAL_MOVES, LINE_AND_ROW_MOVES].concat()
    }
}
