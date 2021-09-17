use crate::pieces::Pieces;
use useful_shit::Players::{self, WHITE, BLACK, NULL};
use crate::pieces::Pieces::{King, Pawn, Rook};
use useful_shit::GameBoard;
use std::fmt::{Display, Formatter};
use crate::agent::Agent;
use crate::moves::{Move, Moves};
#[derive(Debug, Clone)]
pub struct Chess {
    board: Vec<Vec<Pieces>>,
    winner: i32,
    current_player: Players,
    hash: String,
    p1: Agent,
    p2: Agent,
    is_end: bool,

}

impl Chess {
    #[cfg(feature = "run")]
    fn reset_inner(&mut self) {
        self.board = vec![
            vec![Pieces::Rook(-1), Pieces::Knight(-1), Pieces::Bishop(-1), Pieces::Queen(-1), Pieces::King(-1), Pieces::Bishop(-1), Pieces::Knight(-1), Pieces::Rook(-1)],
            vec![Pieces::Pawn(-1);8],
            vec![Pieces::Empty;8],
            vec![Pieces::Empty;8],
            vec![Pieces::Empty;8],
            vec![Pieces::Empty;8],
            vec![Pieces::Pawn(1);8],
            vec![Pieces::Rook(1), Pieces::Knight(1), Pieces::Bishop(1), Pieces::Queen(1), Pieces::King(1), Pieces::Bishop(1), Pieces::Knight(1), Pieces::Rook(1)],
        ];
    }
    pub fn get_current_player(&self) -> Players {
        self.current_player
    }
    #[cfg(feature = "test")]
    fn reset_inner(&mut self) {
        self.current_player = WHITE;
        self.spawn_rook(WHITE);
        self.spawn_rook(BLACK);
    }
    fn spawn_pawn(&mut self, player: Players) {
        match player {
            WHITE => {
                for x in 0..self.board[0].len() {
                    self.board[6][x] = Pieces::Pawn(player, 0);
                }
            },
            BLACK => {
                for x in 0..self.board[0].len() {
                    self.board[1][x] = Pieces::Pawn(player, 0);
                }
            }
            _ => {}
        }
    }
    fn spawn_rook(&mut self, player: Players) {
        match player {
            WHITE => {
                self.board[7][0] = Rook(player);
                self.board[7][7] = Rook(player);
            }
            BLACK => {
                self.board[0][0] = Rook(player);
                self.board[0][7] = Rook(player);
            }
            NULL => {}
        }
    }
    fn spawn_bishop(&mut self, _player: i32) {}
    fn spawn_knight(&mut self, _player: i32) {

    }
    fn spawn_king(&mut self, player: Players) {
        if player == WHITE {
            self.board[7][4] = King(player);
        } else if player == BLACK {
            self.board[0][4] = King(player);
        }
    }
    fn spawn_queen(&mut self, _player: i32) {}
    fn check_pawn_moves(&self, player: Players, x: usize, y: usize, moves: i32) -> Vec<Move> {
        //if white player moves up else player moves down
        let mut available: Vec<Move> = vec![];
        if player == Players::BLACK {
            let mut plays = vec![
                Move::new(Pieces::Pawn(player, moves), (x, y), (x, y+1))];
            if moves == 0 {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x, y), (x, y+2)));
            }
            let diag_x = x.checked_sub(1).unwrap_or(0);
            if self.get_piece_at(y+1, diag_x).get_player() == WHITE {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x,y), (diag_x, y+1)));
            }
            if self.get_piece_at(y+1, x+1).get_player() == WHITE {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x,y), (x+1, y+1)));
            }

            for x in plays {
                if x.is_valid(&self.board) {
                    available.push(x);
                }
            }
        } else if player == Players::WHITE {
            let mut plays = vec![
                Move::new(Pieces::Pawn(player, moves), (x, y), (x, y-1))];
            if moves == 0 {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x, y), (x, y-2)));
            }
            let diag_x = x.checked_sub(1).unwrap_or(0);
            let diag_y = y.checked_sub(1).unwrap_or(0);

            if self.get_piece_at(diag_y, diag_x).get_player() == BLACK {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x,y), (diag_x, diag_y)));
            }
            if  self.get_piece_at(diag_y, x+1).get_player() == BLACK {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x,y), (x+1, diag_y)));
            }
            for x in plays {
                if x.is_valid(&self.board) {
                    available.push(x);
                }
            }
        }
        available
    }
    fn check_king_moves(&self, player: Players, x: usize, y: usize) -> Vec<Move> {
        let mut available: Vec<Move> = vec![];
        for j in x.checked_sub(1).unwrap_or(0)..=x+1 {
            for i in y.checked_sub(1).unwrap_or(0)..=y+1 {
                let play = Move::new(Pieces::King(player), (x, y), (j, i));
                if play.is_valid(&self.board) {
                    available.push(play);
                }
            }
        }
        available
    }
    //TODO FINISH ROOK
    fn can_move(&self, x: usize, y: usize) -> bool {
        match self.board[y][x] {
            Pieces::Pawn(t, i) => {
                return !self.check_pawn_moves(t, x, y, i).is_empty();
            }
            Pieces::King(t) => {
                return !self.check_king_moves(t, x, y).is_empty();
            }
            Pieces::Queen(_t) => {}
            Pieces::Rook(_t) => {}
            Pieces::Bishop(_t) => {}
            Pieces::Knight(_t) => {}
            Pieces::Empty => {}
        }
        false
    }
    fn get_move(&self, x: usize, y: usize) -> Vec<Move> {
        match self.board[y][x] {
            Pieces::Pawn(t, i) => {
                return if t == self.current_player {
                    self.check_pawn_moves(t, x, y, i)
                } else {
                    vec![]
                }
            }
            Pieces::King(t) => {
                return if t == self.current_player {
                    self.check_king_moves(t, x, y)
                } else {
                    vec![]
                }
            }
            Pieces::Queen(_t) => {}
            Pieces::Rook(_t) => {}
            Pieces::Bishop(_t) => {}
            Pieces::Knight(_t) => {}
            Pieces::Empty => {}
        }
        return vec![];
    }
    pub fn new() -> Chess {
        Chess {
            board: vec![vec![Pieces::Empty;8];8],
            winner: 0,
            current_player: WHITE,
            hash: String::new(),
            p1: Agent::new("p1".to_string(), None),
            p2: Agent::new("p2".to_string(), None),
            is_end: false,
        }
    }
    pub fn reset_piece(&self, piece: Pieces) -> Pieces {
        if let Pieces::Knight(_) = piece {
            return Pieces::Knight(self.current_player);
        } else if let Pieces::Bishop(_) = piece {
            return Pieces::Bishop(self.current_player);
        } else if let Pieces::Rook(_) = piece {
            return Pieces::Rook(self.current_player);
        } else if let Pieces::Queen(_) = piece {
            return Pieces::Queen(self.current_player);
        } else if let Pieces::King(_) = piece {
            return Pieces::King(self.current_player);
        } else if let Pieces::Pawn(_, i) = piece {
            return Pieces::Pawn(self.current_player, i);
        }
        piece
    }
    fn save_winner(&self) {
        if self.winner == 1 {
            self.p1.save("winner".to_string());
        } else if self.winner == -1 {
            self.p2.save("winner".to_string());
        }
    }
    fn get_piece_at(&self, i: usize, j: usize) -> Pieces {
        return if i >= 0 && i < self.board.len() && j >= 0 && j < self.board[i].len() {
            self.board[i][j]
        } else {
            King(NULL)
        }
    }
}

impl GameBoard for Chess {
    type Position = Moves;
    type Player = ();
    type Play = Move;

    fn available_positions(&self, player: Players) -> Self::Position {

        let mut positions: Vec<Move> = vec![];
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    Pieces::Pawn(_, _) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::King(_t) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::Queen(t) => {
                        if t == player {
                            //check vertical
                            let mut blocked = false;
                            for y in i..self.board.len() {
                                if self.get_piece_at(y, j) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (j, y)));
                                } else {
                                    blocked = true;
                                }
                            }
                            blocked = false;
                            for y in (0..i).rev() {
                                if self.get_piece_at(y, j) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (j, y)));
                                } else {
                                    blocked = true;
                                }
                            }

                            //check horizontal
                            blocked = false;
                            for x in j..self.board[i].len() {
                                if self.get_piece_at(i, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (x, i)));
                                } else {
                                    blocked = true;
                                }
                            }
                            blocked = false;
                            for x in (0..j).rev() {
                                if self.get_piece_at(i, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (x, i)));
                                } else {
                                    blocked = true;
                                }
                            }
                            //check diagonal
                            //should be good
                            blocked = false;
                            for x in j..self.board[i].len() {
                                //println!("{} {}", x, i as i32-(x-j) as i32);
                                //println!("{} {:?}", (x, x+i-j), self.get_piece_at(x+i-j, x));
                                if self.get_piece_at(x + i - j, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (x, x + i - j)));
                                } else {
                                    blocked = true
                                }
                            }
                            //should be good
                            blocked = false;
                            for x in j..self.board[i].len() {
                                if (i as i32 - (x - j) as i32) < 0 {
                                    break;
                                }
                                //TODO keep an eye on this
                                if self.get_piece_at((i as i32 - (x - j) as i32) as usize, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (x, (i as i32 - (x - j) as i32) as usize)));
                                } else {
                                    blocked = true;
                                }
                            }
                            //should be good
                            blocked = false;
                            for x in (0..j).rev() {
                                let y = x as i32 + i as i32 - j as i32;
                                if y < 0 {
                                    break;
                                }
                                let y = y as usize;
                                if self.get_piece_at(y, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (x, y)));
                                } else {
                                    blocked = true;
                                }
                            }
                            //should be good
                            blocked = false;
                            for x in (0..j).rev() {
                                let y = i as i32 - x as i32 + j as i32;
                                if self.get_piece_at(y as usize, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Queen(t), (j, i), (x, y as usize)));
                                } else {
                                    blocked = true;
                                }
                            }
                        }
                    }
                    Pieces::Rook(t) => {
                        if t == player {
                            let mut blocked = false;
                            for x in (0..j).rev() {
                                if self.get_piece_at(i, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Rook(t), (j, i), (x, i)));
                                } else {
                                    blocked = true;
                                }
                            }
                            for x in j..self.board[i].len() {
                                if self.get_piece_at(i, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Rook(t), (j, i), (x, i)));
                                } else {
                                    blocked = true;
                                }
                            }
                            for y in (0..i).rev() {
                                if self.get_piece_at(y, j) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Rook(t), (j, i), (j, y)));
                                } else {
                                    blocked = true;
                                }
                            }
                            for y in i..self.board.len() {
                                if self.get_piece_at(y, j) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Rook(t), (j, i), (j, y)));
                                } else {
                                    blocked = true;
                                }
                            }
                        }
                    }
                    Pieces::Bishop(t) => {
                        if t == player {
                            let mut blocked = false;
                            for x in j..self.board[i].len() {
                                //println!("{} {}", x, i as i32-(x-j) as i32);
                                //println!("{} {:?}", (x, x+i-j), self.get_piece_at(x+i-j, x));
                                if self.get_piece_at(x + i - j, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Bishop(t), (j, i), (x, x + i - j)));
                                } else {
                                    blocked = true
                                }
                            }
                            //should be good
                            blocked = false;
                            for x in j..self.board[i].len() {
                                if (i as i32 - (x - j) as i32) < 0 {
                                    break;
                                }
                                //TODO keep an eye on this
                                if self.get_piece_at((i as i32 - (x - j) as i32) as usize, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Bishop(t), (j, i), (x, (i as i32 - (x - j) as i32) as usize)));
                                } else {
                                    blocked = true;
                                }
                            }
                            //should be good
                            blocked = false;
                            for x in (0..j).rev() {
                                let y = (x as i32 + i as i32 - j as i32);
                                if y < 0 {
                                    break;
                                }
                                let y = y as usize;
                                if self.get_piece_at(y, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Bishop(t), (j, i), (x, y)));
                                } else {
                                    blocked = true;
                                }
                            }
                            //should be good
                            blocked = false;
                            for x in (0..j).rev() {
                                let y = i as i32 - x as i32 + j as i32;
                                if self.get_piece_at(y as usize, x) == Pieces::Empty && !blocked {
                                    positions.push(Move::new(Pieces::Bishop(t), (j, i), (x, y as usize)));
                                } else {
                                    blocked = true;
                                }
                            }
                        }
                    }
                    Pieces::Knight(t) => {
                        if t == player {
                            //have to hard code this one
                            let x = j as i32;
                            let y = i as i32;
                            let tl = ((x - 1) as usize, (y - 2) as usize);
                            let tr = ((x + 1) as usize, (y - 2) as usize);

                            let mr1 = ((x + 2) as usize, (y - 1) as usize);
                            let ml1 = ((x - 2) as usize, (y - 1) as usize);

                            let mr2 = ((x + 2) as usize, (y + 1) as usize);
                            let ml2 = ((x - 2) as usize, (y + 1) as usize);

                            let bl = ((x - 1) as usize, (y + 2) as usize);
                            let br = ((x + 1) as usize, (y + 2) as usize);
                            if self.get_piece_at(tl.1, tl.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), tl));
                            }
                            if self.get_piece_at(tr.1, tr.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), tr));
                            }
                            if self.get_piece_at(mr1.1, mr1.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), mr1));
                            }
                            if self.get_piece_at(ml1.1, ml1.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), ml1));
                            }
                            if self.get_piece_at(mr2.1, mr2.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), mr2));
                            }
                            if self.get_piece_at(ml2.1, ml2.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), ml2));
                            }
                            if self.get_piece_at(bl.1, bl.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), bl));
                            }
                            if self.get_piece_at(br.1, br.0) == Pieces::Empty {
                                positions.push(Move::new(Pieces::Knight(t), (j, i), br));
                            }
                        }
                    }
                    Pieces::Empty => {}
                }
            }
        }
        Moves::new(positions)
    }
    fn check_winner(&mut self) -> Option<Self::Player> {
        todo!()
    }
    fn give_reward(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        self.reset_inner();
    }

    fn update(&mut self, play: Self::Play) -> Result<(), ()> {
        if let Pawn(p, i) = play.get_piece() {
            self.board[play.get_new_position().1][play.get_new_position().0] = Pawn(p, i+1);
        } else {
            self.board[play.get_new_position().1][play.get_new_position().0] = play.get_piece();
        }
        self.board[play.get_current_position().1][play.get_current_position().0] = Pieces::Empty;

        let new_player = self.current_player as i32*-1;
        self.current_player = num::FromPrimitive::from_i32(new_player).unwrap();

        Ok(())
    }

}


impl Display for Chess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.board.len() {

            write!(f, "{} ", i+1);
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    Pieces::Pawn(t, _) => {
                        match t {
                            WHITE => {
                                write!(f, "WP ");
                            }
                            BLACK => {
                                write!(f, "BP ");
                            }
                            NULL => {}
                        }
                    }
                    Pieces::King(t) => {
                        match t {
                            WHITE => {
                                write!(f, "WK ");
                            }
                            BLACK => {
                                write!(f, "BK ");
                            }
                            NULL => {}
                        }
                    }
                    Pieces::Queen(t) => {
                        match t {
                            WHITE => {
                                write!(f, "WQ ");
                            }
                            BLACK => {
                                write!(f, "BQ ");
                            }
                            NULL => {}
                        }
                    }
                    Pieces::Rook(t) => {
                        match t {
                            WHITE => {
                                write!(f, "WR ");
                            }
                            BLACK => {
                                write!(f, "BR ");
                            }
                            NULL => {}
                        }
                    }
                    Pieces::Bishop(t) => {
                        match t {
                            WHITE => {
                                write!(f, "WB ");
                            }
                            BLACK => {
                                write!(f, "BB ");
                            }
                            NULL => {}
                        }
                    }
                    Pieces::Knight(t) => {
                        match t {
                            WHITE => {
                                write!(f, "WH ");
                            }
                            BLACK => {
                                write!(f, "BH ");
                            }
                            NULL => {}
                        }
                    }
                    Pieces::Empty => {
                        write!(f, "   ");
                    }
                }
            }
            write!(f, "\n");
        }
        write!(f, "  A  B  C  D  E  F  G  H");
        Ok(())
    }
}