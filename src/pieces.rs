use useful_shit::Players::*;
use std::str::FromStr;
use useful_shit::Players;

impl FromStr for Pieces {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.to_string().to_lowercase();
        if string.contains("pawn") {
            return Ok(Pieces::Pawn(NULL, 0));
        } else if string.contains("king") {
            return Ok(Pieces::King(NULL));
        } else if string.contains("queen") {
            return Ok(Pieces::Queen(NULL));
        } else if string.contains("rook") {
            return Ok(Pieces::Rook(NULL));
        } else if string.contains("bishop") {
            return Ok(Pieces::Bishop(NULL));
        } else if string.contains("knight") {
            return Ok(Pieces::Knight(NULL));
        }

        Err(())
    }
}
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Pieces {
    Pawn(Players, i32),
    King(Players),
    Queen(Players),
    Rook(Players),
    Bishop(Players),
    Knight(Players),
    Empty,
}
impl Pieces {
    pub(crate) fn get_player(self) -> Players {
        return match self {
            Pieces::Pawn(t, _) => {
                t
            }
            Pieces::King(t) => {
                t
            }
            Pieces::Queen(t) => {
                t
            }
            Pieces::Rook(t) => {
                t
            }
            Pieces::Bishop(t) => {
                t
            }
            Pieces::Knight(t) => {
                t
            }
            Pieces::Empty => {
                NULL
            }
        }
    }
    fn get_moves(&self) -> i32 {
        if let Pieces::Pawn(_, i) = self {
            return *i;
        }
        -1
    }
}