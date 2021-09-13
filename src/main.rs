use useful_shit::GameBoard;
use std::collections::HashMap;
use std::fs::File;
use std::fmt::{Display, Formatter};

impl Display for Chess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.board.len() {

            write!(f, "{} ", i+1);
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    Pieces::Pawn(_) => {
                        write!(f, "P ");
                    }
                    Pieces::King(_) => {
                        write!(f, "K ");
                    }
                    Pieces::Queen(_) => {
                        write!(f, "Q ");
                    }
                    Pieces::Rook(_) => {
                        write!(f, "R ");
                    }
                    Pieces::Bishop(_) => {
                        write!(f, "B ");
                    }
                    Pieces::Knight(_) => {
                        write!(f, "H ");
                    }
                    Pieces::Empty => {
                        write!(f, "  ");
                    }
                }
            }
            write!(f, "\n");
        }
        write!(f, "  A B C D E F G H");
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
    Pawn(i32),
    King(i32),
    Queen(i32),
    Rook(i32),
    Bishop(i32),
    Knight(i32),
    Empty,
}
#[derive(Debug, Clone)]
struct Move {
    piece: Pieces,
    current_position: String,
    new_position: String,

}
impl Move {
    fn new(piece: Pieces, current: String, new: String) -> Move {
        Move {
            piece,
            current_position: current,
            new_position: new,
        }
    }
}
struct Agent {
    states: Vec<String>,
    lr: f32,
    exp_rate: f32,
    decay_gamma: f32,
    states_value: HashMap<String, f32>,
    name: String,
}

struct Chess {
    board: Vec<Vec<Pieces>>,
    winner: i32,
    current_player: i32,
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

    fn new() -> Chess {
        Chess {
            board: vec![vec![Pieces::Empty;8];8],
            winner: 0,
            current_player: 1,
            hash: String::new(),
            p1: Agent::new("p1".to_string(), None),
            p2: Agent::new("p2".to_string(), None),
            is_end: false,
        }
    }
    fn save_winner(&self) {
        if self.winner == 1 {
            self.p1.save("winner".to_string());
        } else if self.winner == -1 {
            self.p2.save("winner".to_string());
        }
    }
    fn get_piece_at(&self, i: usize, j: usize) -> Pieces {
        *self.board.get(i).unwrap_or(&vec![Pieces::King(0); 8]).get(j).unwrap_or(&Pieces::King(0))
    }
}

impl GameBoard for Chess {
    type Position = Move;
    type Player = ();

    fn available_positions(&self) -> Vec<Self::Position> {
        let mut positions: Vec<Self::Position> = vec![];
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                //white
                match self.board[i][j] {
                    Pieces::Pawn(t) => {
                        if t == -1 {
                            if i < self.board.len() && self.board[i+1][j] == Pieces::Empty {
                                positions.push(Move::new(Pieces::Pawn(t), (j, i).to_rank_file(), (j, i+1).to_rank_file()))
                            }
                        } else if t == 1 {
                            if i > 0 && self.board[i-1][j] == Pieces::Empty {
                                positions.push(Move::new(Pieces::Pawn(t), (j, i).to_rank_file(), (j, i-1).to_rank_file()))
                            }
                        }
                    }
                    Pieces::King(t) => {
                        for y in i.checked_sub(1).unwrap_or(0)..=i+1 {
                            for x in j.checked_sub(1).unwrap_or(0)..=j+1 {
                                if self.get_piece_at(y, x) == Pieces::Empty {
                                    positions.push(Move::new(Pieces::King(t), (j, i).to_rank_file(), (x, y).to_rank_file()));
                                }
                            }
                        }
                    }
                    Pieces::Queen(t) => {
                        //check vertical
                        let mut blocked = false;
                        for y in i..self.board.len() {
                            if self.get_piece_at(y, j) == Pieces::Empty && !blocked {
                                positions.push(Move::new(Pieces::Queen(t), (j, i).to_rank_file(), (j, y).to_rank_file()));
                            } else {
                                blocked = true;
                            }
                        }
                        blocked = false;
                        for y in (0..i).rev() {
                            if self.get_piece_at(y, j) == Pieces::Empty && !blocked {
                                positions.push(Move::new(Pieces::Queen(t), (j, i).to_rank_file(), (j, y).to_rank_file()));
                            } else {
                                blocked = true;
                            }
                        }

                        //check horizontal
                        blocked = false;
                        for x in j..self.board[i].len() {
                            if self.get_piece_at(i, x) == Pieces::Empty && !blocked {
                                positions.push(Move::new(Pieces::Queen(t), (j, i).to_rank_file(), (x, i).to_rank_file()));
                            } else {
                                blocked = true;
                            }
                        }
                        blocked = false;
                        for x in (0..j).rev() {
                            if self.get_piece_at(i, x) == Pieces::Empty && !blocked {
                                positions.push(Move::new(Pieces::Queen(t), (j, i).to_rank_file(), (x, i).to_rank_file()));
                            } else {
                                blocked = true;
                            }
                        }
                        //check diagonal
                        blocked = false;
                        for x in j..self.board[i].len() {
                            println!("{} {}", x, i as i32-(x-j) as i32);
                            //println!("{} {:?}", (x, x+i-j).to_rank_file(), self.get_piece_at(x+i-j, x));
                            if self.get_piece_at(x+i-j, x) == Pieces::Empty && !blocked {
                                positions.push(Move::new(Pieces::Queen(t), (j, i).to_rank_file(), (x, x+i-j).to_rank_file()));
                            }  else {
                                blocked = true
                            }
                        }
                        blocked = false;
                        for x in j..self.board[i].len() {
                            //TODO keep an eye on this
                            if self.get_piece_at((i as i32 - (x-j) as i32) as usize, x) == Pieces::Empty && !blocked {
                                positions.push(Move::new(Pieces::Queen(t), (j, i).to_rank_file(), (x, (i as i32 - (x-j) as i32) as usize).to_rank_file()));
                            } else {
                                blocked = true;
                            }
                         }
                        //TODO Do other diagonal
                    }
                    Pieces::Rook(t) => {}
                    Pieces::Bishop(t) => {}
                    Pieces::Knight(t) => {}
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

    fn update(&mut self, play: Self::Position) -> Result<(), ()> {
        todo!()
    }
}
fn main() {
    let mut game = Chess::new();
    game.reset();
    println!("{}", game);
    println!("{:?}", game.available_positions());
}
