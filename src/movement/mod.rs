use crate::{Move, MoveError, square_to_index};

#[derive(PartialEq, Debug)]
pub struct Movement {
    pub from: Move,
    pub to: Move,
    pub capture: bool,
}

impl Movement {
    pub fn parse_movement_notation(movement: &str) -> Result<Movement, MoveError> {
        let parts: Vec<&str> = movement.split_ascii_whitespace().collect();
        let (from_notation, to_notation): (&str, &str) = if parts.len() != 2 {
            return Err(MoveError::IllegalMovementNotation)
        } else {
            (parts[0], parts[1])
        };

        let from = square_to_index(from_notation);
        let to = square_to_index(to_notation);

        if !Movement::is_legal_move(to) {
            return Err(MoveError::ToSquareOutOfBoard)
        }

        if !Movement::is_legal_move(from) {
            return Err(MoveError::FromSquareOutOfBoard)
        }

        Ok(Movement {
            from,
            to,
            capture: false,
        })
    }

    fn is_legal_move(square: Move) -> bool {
        square.0 <= 8 &&
            square.1 <= 8
    }
}