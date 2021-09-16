#![feature(string_remove_matches)]


use useful_shit::{GameBoard, input, Players};
use std::collections::HashMap;
use std::fs::File;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use useful_shit::Players::{WHITE, BLACK, NULL};


trait ToPosition {
    fn to_position(&self) -> (usize, usize);
}
impl ToPosition for String {
    fn to_position(&self) -> (usize, usize) {

        let xy = self.split("").collect::<Vec<&str>>();
        let mut vec = vec![];
        for x in xy {
            if !x.is_empty() {
                vec.push(x);
            }
        }
        let xy = vec;
        let x = &xy[0].to_string();
        let y = &xy[1];
        let rank: usize = (x.chars().collect::<Vec<char>>()[0] as u8 - 65) as usize;
        let file: usize = y.parse().unwrap_or(0) - 1;
        (rank, file)
    }
}
impl FromStr for Pieces {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string = s.to_string().to_lowercase();
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
trait Coords {
    fn to_rank_file(&self) -> String;
    
}
impl Coords for (usize, usize) {
    fn to_rank_file(&self) -> String {
        let rank = self.0 as u8+ 65;
        if rank > 72 {
            panic!("Invalid Coord");
        }
        format!("{}{}", rank as char, self.1+1)
    }
}
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
enum Pieces {
    Pawn(Players, i32),
    King(Players),
    Queen(Players),
    Rook(Players),
    Bishop(Players),
    Knight(Players),
    Empty,
}
impl Pieces {
    fn get_player(self) -> Players {
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
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
struct Move {
    piece: Pieces,
    current_position: (usize, usize),
    new_position: (usize, usize),

}
impl Move {
    fn new(piece: Pieces, current: (usize, usize), new: (usize, usize)) -> Move {
        Move {
            piece,
            current_position: current,
            new_position: new,
        }
    }
    fn is_valid(&self, board: &Vec<Vec<Pieces>>) -> bool {
        //X and Y values are less than the board length and are positive
        (self.new_position.0 < board[0].len() && self.new_position.0 >= 0)
            && (self.new_position.1 < board.len() && self.new_position.1 >= 0)
    }
}
#[derive(Clone, Debug)]
struct Agent {
    states: Vec<String>,
    lr: f32,
    exp_rate: f32,
    decay_gamma: f32,
    states_value: HashMap<String, f32>,
    name: String,
}
#[derive(Debug, Clone)]
struct Chess {
    board: Vec<Vec<Pieces>>,
    winner: i32,
    current_player: Players,
    hash: String,
    p1: Agent,
    p2: Agent,
    is_end: bool,

}

impl Agent {
    fn new(name: String, exp_rate: Option<f32>) -> Agent {
        Agent {
            states: vec![],
            lr: 0.2,
            exp_rate: exp_rate.unwrap_or(0.3),
            decay_gamma: 0.9,
            states_value: HashMap::new(),
            name,
        }

    }
    fn save(&self, filename: String) {
        let mut file = File::create(filename).expect("Could not create policy");

        serde_json::to_writer(file, &self.states_value);
    }
    fn try_load(&mut self, filename: String) {
        let mut file = File::open(filename);
        if file.is_err() {
            eprintln!("Failed to load agent");
            return;
        }
        self.states_value = serde_json::from_reader(file.unwrap()).unwrap();
    }
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
    #[cfg(feature = "test")]
    fn reset_inner(&mut self) {
        self.current_player = WHITE;
        self.spawn_pawn(BLACK);
        self.spawn_pawn(WHITE);
    }
    fn spawn_pawn(&mut self, player: Players) {
        if player == WHITE {
            for x in 0..self.board[0].len() {
                self.board[6][x] = Pieces::Pawn(player, 0);
            }
        }
        if player == BLACK {
            for x in 0..self.board[0].len() {
                self.board[1][x] = Pieces::Pawn(player, 0);
            }
        }
    }
    fn spawn_rook(&mut self, player: i32) {}
    fn spawn_bishop(&mut self, player: i32) {}
    fn spawn_knight(&mut self, player: i32) {

    }
    fn spawn_king(&mut self, player: i32) {}
    fn spawn_queen(&mut self, player: i32) {}
    fn check_pawn_moves(&self, player: Players, x: usize, y: usize, moves: i32) -> Vec<Move> {
        //if white player moves up else player moves down
        let mut available: Vec<Move> = vec![];
        if player == Players::BLACK {
            let mut plays = vec![
                Move::new(Pieces::Pawn(player, moves), (x, y), (x, y+1))];
            if moves == 0 {
                plays.push(Move::new(Pieces::Pawn(player, moves), (x, y), (x, y+2)));
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
        for j in x-1..=x+1 {
            for i in y-1..=y+1 {
                let play = Move::new(Pieces::King(player), (x, y), (j, i));
                if play.is_valid(&self.board) {
                    available.push(play);
                }
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
            Pieces::Queen(t) => {}
            Pieces::Rook(t) => {}
            Pieces::Bishop(t) => {}
            Pieces::Knight(t) => {}
            Pieces::Empty => {}
        }
        false
    }
    fn get_move(&self, x: usize, y: usize) -> Vec<Move> {
        match self.board[y][x] {
            Pieces::Pawn(t, i) => {
                return self.check_pawn_moves(t, x, y, i);
            }
            Pieces::King(t) => {}
            Pieces::Queen(t) => {}
            Pieces::Rook(t) => {}
            Pieces::Bishop(t) => {}
            Pieces::Knight(t) => {}
            Pieces::Empty => {}
        }
        return vec![];
    }
    fn new() -> Chess {
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
    fn reset_piece(&self, piece: Pieces) -> Pieces {
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
        *self.board.get(i).unwrap_or(&vec![Pieces::King(NULL); 8]).get(j).unwrap_or(&Pieces::King(NULL))
    }
}

impl GameBoard for Chess {
    type Position = Move;
    type Player = ();

    fn available_positions(&self, player: Players) -> Vec<Self::Position> {

        let mut positions: Vec<Self::Position> = vec![];
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    Pieces::Pawn(_, _) => {
                        positions.append(&mut self.get_move(j, i));
                    }
                    Pieces::King(t) => {
                        if t == player {
                            for y in i.checked_sub(1).unwrap_or(0)..=i + 1 {
                                for x in j.checked_sub(1).unwrap_or(0)..=j + 1 {
                                    if self.get_piece_at(y, x) == Pieces::Empty {
                                        positions.push(Move::new(Pieces::King(t), (j, i), (x, y)));
                                    }
                                }
                            }
                        }
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
                                let mut y = (x as i32 + i as i32 - j as i32);
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
                                let mut y = (x as i32 + i as i32 - j as i32);
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
        positions
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

    fn update(&mut self, play: Self::Position) -> Result<(), ()> {
        let available_plays = self.available_positions(self.current_player);
        if !available_plays.contains(&play) {
            eprintln!("Not a valid play");
            return Err(());
        }
        self.board[play.current_position.1][play.current_position.0] = Pieces::Empty;
        self.board[play.new_position.1][play.new_position.0] = play.piece;
        let new_player = self.current_player as i32*-1;
        self.current_player = num::FromPrimitive::from_i32(new_player).unwrap();

        Ok(())
    }

}


fn main() {
    let mut game = Chess::new();
    game.reset();
    loop {
        println!("{}", game);
        println!("Player {:?}", game.current_player);
        println!("{:?}", game.available_positions(game.current_player));
        let mut player_piece: Pieces = input("Piece: ");
        let mut current_place: String = input("Current Position: ");
        let mut new_place: String = input("New Position: ");
        player_piece = game.reset_piece(player_piece);
        println!("Attempting to move {:?} from {:?} to {:?}", player_piece, current_place.to_position(), new_place.to_position());
        let player_move = Move::new(player_piece, current_place.to_position(), new_place.to_position());
        game.update(player_move);

        println!("{}", game);
        println!("{:?}", game.available_positions(game.current_player));
    }

}

//TODO pawn two square movement
//TODO the ability to take