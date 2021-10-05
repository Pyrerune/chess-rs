use crate::pieces::Pieces;
use crate::traits::Coords;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct Moves(Vec<Move>);

impl Display for Moves {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let plays = &self.0;
        let mut i = 0;
        for play in plays {
            write!(
                f,
                "{}. {} {:?} to {}\n",
                i,
                play.current_position.to_rank_file(),
                play.piece,
                play.new_position.to_rank_file()
            );
            i += 1;
        }
        Ok(())
    }
}

impl Moves {
    pub fn new(inner: Vec<Move>) -> Moves {
        Moves(inner)
    }

    pub fn get_inner(&self) -> &Vec<Move> {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Move {
    piece: Pieces,
    current_position: (usize, usize),
    new_position: (usize, usize),
}

impl Move {
    pub fn new(piece: Pieces, current: (usize, usize), new: (usize, usize)) -> Move {
        Move {
            piece,
            current_position: current,
            new_position: new,
        }
    }

    pub fn is_valid(&self, board: &Vec<Vec<Pieces>>) -> bool {
        //X and Y values are less than the board length and are positive
        (self.new_position.0 < board[0].len() && self.new_position.0 >= 0)
            && (self.new_position.1 < board.len() && self.new_position.1 >= 0)
            && (board[self.new_position.1][self.new_position.0].get_player()
                != self.piece.get_player())
    }

    pub fn get_piece(&self) -> Pieces {
        self.piece
    }

    pub fn set_piece(&mut self, piece: Pieces) {
        self.piece = piece;
    }

    pub fn get_new_position(&self) -> (usize, usize) {
        self.new_position
    }

    pub fn get_current_position(&self) -> (usize, usize) {
        self.current_position
    }
}
