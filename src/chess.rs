use crate::pieces::Pieces;
use useful_shit::Players::{self, WHITE, BLACK, NULL};
use crate::pieces::Pieces::{King, Pawn, Rook, Queen, Bishop, Knight};
use useful_shit::GameBoard;
use std::fmt::{Display, Formatter};
use crate::agent::Agent;
use crate::moves::{Move, Moves};
use std::mem::take;

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

        self.spawn_rook(WHITE);
        self.spawn_pawn(WHITE);
        self.spawn_king(WHITE);
        self.spawn_queen(WHITE);
        self.spawn_bishop(WHITE);
        self.spawn_knight(WHITE);
        self.spawn_knight(BLACK);
        self.spawn_bishop(BLACK);
        self.spawn_rook(BLACK);
        self.spawn_pawn(BLACK);
        self.spawn_king(BLACK);
        self.spawn_queen(BLACK);
    }
    pub fn get_current_player(&self) -> Players {
        self.current_player
    }
    #[cfg(feature = "test")]
    fn reset_inner(&mut self) {
        self.current_player = WHITE;
        self.spawn_king(WHITE);
        //self.spawn_pawn(BLACK);
        self.board[6][3] = Pawn(BLACK, 0);
        self.spawn_knight(WHITE);
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
    fn spawn_bishop(&mut self, player: Players) {
        match player {
            WHITE => {
                self.board[7][2] = Bishop(player);
                self.board[7][5] = Bishop(player);
            }
            BLACK => {
                self.board[0][2] = Bishop(player);
                self.board[0][5] = Bishop(player);
            }
            _ => {}
        }
    }
    fn spawn_knight(&mut self, player: Players) {
        match player {
            WHITE => {
                self.board[7][1] = Knight(player);
                self.board[7][6] = Knight(player);
            }
            BLACK => {
                self.board[0][1] = Knight(player);
                self.board[0][6] = Knight(player);
            }
            _ => {}
        }
    }
    fn spawn_king(&mut self, player: Players) {
        if player == WHITE {
            self.board[7][4] = King(player);
        } else if player == BLACK {
            self.board[0][4] = King(player);
        }
    }
    fn spawn_queen(&mut self, player: Players) {
        match player {
            WHITE => {
                self.board[7][3] = Queen(player);
            }
            BLACK => {
                self.board[0][3] = Queen(player);
            }
            _ => {}
        }
    }
    fn check_pawn_moves(&self, player: Players, x: usize, y: usize, moves: i32) -> Vec<Move> {
        //if white player moves up else player moves down
        let mut available: Vec<Move> = vec![];
        if player == Players::BLACK {
            let first = Move::new(Pieces::Pawn(player, moves), (x, y), (x, y+1));
            let mut plays = vec![first];
            if moves == 0 && self.is_pos_empty(first) {
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
            let first = Move::new(Pieces::Pawn(player, moves), (x, y), (x, y-1));
            let mut plays = vec![first];
            if moves == 0 && self.is_pos_empty(first) {
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
    fn check_rook_moves(&self, player: Players, x: usize, y: usize) -> Vec<Move> {
        let mut available: Vec<Move> = vec![];
        //X start from the rook going to 0
        let mut j = x.checked_sub(1).unwrap_or(0);
        let mut play = Move::new(Pieces::Rook(player), (x, y), (j, y));
        let mut empty = self.is_pos_empty(play);
        while empty {
            play = Move::new(Pieces::Rook(player), (x,y), (j, y));
            empty = self.is_pos_empty(play);
            if j > 0 {
                j -=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }

        //X start from the rook going to the end of the board
        j = x+1;
        play = Move::new(Pieces::Rook(player), (x, y), (j, y));
        empty = self.is_pos_empty(play);
        while empty {
            play = Move::new(Pieces::Rook(player), (x,y), (j, y));
            empty = self.is_pos_empty(play);
            if j < self.board.len() {
                j +=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }

        //Y start from the rook going to 0
        let mut i = y.checked_sub(1).unwrap_or(0);
        play = Move::new(Pieces::Rook(player), (x,y), (x, i));
        empty = self.is_pos_empty(play);
        while empty {
            play = Move::new(Pieces::Rook(player), (x,y), (x, i));
            empty = self.is_pos_empty(play);
            if i > 0 {
                i-=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }
        i = y+1;
        play = Move::new(Pieces::Rook(player), (x,y), (x, i));
        empty = self.is_pos_empty(play);
        while empty {
            play = Move::new(Pieces::Rook(player), (x,y), (x, i));
            empty = self.is_pos_empty(play);
            if i < self.board.len() {
                i+=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }
        available
    }
    fn check_queen_moves(&self, player: Players, x: usize, y: usize) -> Vec<Move> {
        let mut available: Vec<Move> = vec![];
        //reuse rook code for straight lines
        let mut lines = self.check_rook_moves(player, x, y);
        for x in lines {
            let mut copy = x.clone();
            copy.set_piece(Queen(player));
            available.push(copy);
        }
        let mut diags = self.check_bishop_moves(player, x, y);
        for y in diags {
            let mut copy = y.clone();
            copy.set_piece(Queen(player));
            available.push(copy);
        }
        available
    }
    fn check_bishop_moves(&self, player: Players, x: usize, y: usize) -> Vec<Move> {
        let mut available: Vec<Move> = vec![];
        //check moves from current x to zero
        //y is calculated the same as x
        let mut j = x;
        let mut i = y;
        let mut empty = true;
        let mut play: Move;
        if j <= 0 || i <= 0 {
            empty = false;
        } else {
            i-=1;
            j-=1;
            play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
            empty = self.is_pos_empty(play);
        }
        //diagonal to the left and up
        while empty {
            play = Move::new(Pieces::Bishop(player), (x,y), (j, i));
            empty = self.is_pos_empty(play);
            if j > 0 && i > 0 {
                j -=1;
                i -=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }
        j = x;
        i = y;
        //diagonal left and down
        play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
        if j <= 0 || i >= self.board.len() {
            empty = false;
        } else {
            i+=1;
            j-=1;
            play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
            empty = self.is_pos_empty(play);
        }
        while empty {
            play = Move::new(Pieces::Bishop(player), (x,y), (j, i));
            empty = self.is_pos_empty(play);
            if j > 0 && i < self.board.len() {
                j -=1;
                i +=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }
        j = x;
        i = y;
        //diagonal right and up
        play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
        if j >= self.board.len() || i <= 0 {
            empty = false;
        } else {
            i-=1;
            j+=1;
            play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
            empty = self.is_pos_empty(play);
        }
        while empty {
            play = Move::new(Pieces::Bishop(player), (x,y), (j, i));
            empty = self.is_pos_empty(play);
            if j < self.board.len() && i > 0 {
                j +=1;
                i -=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }
        j = x;
        i = y;
        //diagonal right and down
        play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
        if j >= self.board.len() || i >= self.board.len() {
            empty = false;
        } else {
            i+=1;
            j+=1;
            play = Move::new(Pieces::Bishop(player), (x, y), (j, i));
            empty = self.is_pos_empty(play);
        }
        while empty {
            play = Move::new(Pieces::Bishop(player), (x,y), (j, i));
            empty = self.is_pos_empty(play);
            if j < self.board.len() && i < self.board.len() {
                j +=1;
                i +=1;
            } else {
                empty = false;
            }
            if play.is_valid(&self.board) {
                available.push(play);
            }
        }
        available
    }
    fn check_knight_moves(&self, player: Players, x: usize, y: usize) -> Vec<Move> {
        let mut available: Vec<Move> = vec![];
        let mut i = y as i32;
        let mut j = x as i32;
        let pos = [((j - 1) as usize, (i - 2) as usize), ((j + 1) as usize, (i - 2) as usize),
            ((j + 2) as usize, (i - 1) as usize), ((j - 2) as usize, (i - 1) as usize),
            ((j + 2) as usize, (i + 1) as usize), ((j - 2) as usize, (i + 1) as usize),
            ((j - 1) as usize, (i + 2) as usize), ((j + 1) as usize, (i + 2) as usize)];
        for position in pos {
            let play = Move::new(Knight(player), (x, y), position);
            if play.is_valid(&self.board) && self.is_pos_empty(play) {
                available.push(play);
            }
        }
        available
    }
    fn can_move(&self, x: usize, y: usize) -> bool {
        match self.board[y][x] {
            Pieces::Pawn(t, i) => {
                return !self.check_pawn_moves(t, x, y, i).is_empty();
            }
            Pieces::King(t) => {
                return !self.check_king_moves(t, x, y).is_empty();
            }
            Pieces::Queen(t) => {
                return !self.check_queen_moves(t, x, y).is_empty();
            }
            Pieces::Rook(t) => {
                return !self.check_rook_moves(t, x, y).is_empty();
            }
            Pieces::Bishop(t) => {
                return !self.check_bishop_moves(t, x, y).is_empty();
            }
            Pieces::Knight(t) => {
                return !self.check_knight_moves(t, x, y).is_empty();
            }
            Pieces::Empty => {}
        }
        false
    }
    fn get_move(&self, player: Players, x: usize, y: usize) -> Vec<Move> {
        match self.board[y][x] {
            Pieces::Pawn(t, i) => {
                return if t == player {
                    self.check_pawn_moves(t, x, y, i)
                } else {
                    vec![]
                }
            }
            Pieces::King(t) => {
                return if t == player {
                    self.check_king_moves(t, x, y)
                } else {
                    vec![]
                }
            }
            Pieces::Queen(t) => {
                return if t == player {
                    self.check_queen_moves(t, x, y)
                } else {
                    vec![]
                }
            }
            Pieces::Rook(t) => {
                return if t == player {
                    self.check_rook_moves(t, x, y)
                } else {
                    vec![]
                }
            }
            Pieces::Bishop(t) => {
                return if t == player {
                    self.check_bishop_moves(t, x, y)
                } else {
                    vec![]
                }
            }
            Pieces::Knight(t) => {
                return if t == player {
                    self.check_knight_moves(t, x, y)
                } else {
                    vec![]
                }
            }
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
    fn is_pos_empty(&self, play: Move) -> bool {
        let position = play.get_new_position();
        let player = self.get_piece_at(position.1, position.0).get_player();
        let you = play.get_piece().get_player();
        player != you || player ==NULL
    }
    fn get_king_pos(&self, player: Players) -> (usize, usize) {

        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == King(player) {
                    return (j, i);
                }
            }
        }
        (usize::MAX, usize::MAX)
    }
    //TODO IMPLEMENT CHECKING
    pub fn check_king_safety(&self, player: Players) -> bool {
        let mut moves: Moves = Moves::new(vec![]);
        let mut king_pos = self.get_king_pos(player);
        match player {
            WHITE => {
                moves = self.available_positions(BLACK);
            }
            BLACK => {
                moves = self.available_positions(WHITE);
            }
            NULL => {}
        }
        let moves = moves.get_inner();
        for play in moves {
            if play.get_new_position() == king_pos {
                return false;
            }
        }
        true
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
                positions.append(&mut self.get_move(player, j, i));
                /*match self.board[i][j] {
                    Pieces::Pawn(_, _) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::King(_t) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::Queen(_t) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::Rook(_t) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::Bishop(t) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::Knight(t) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::Empty => {}
                }*/
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