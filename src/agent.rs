use std::collections::HashMap;
use std::fs::File;
use useful_shit::{AiFunctions, compress, Players, GameBoard, Reversed};
use rand::{thread_rng, Rng};
use crate::moves::Move;
use crate::pieces::Pieces::King;
use useful_shit::Players::NULL;
use crate::chess::Chess;
use std::io::{Write, BufWriter, Read};
use std::fs::OpenOptions;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Data {
    round: i32,
    data: HashMap<String, f32>
}
#[derive(Clone, Debug)]
pub struct Agent {
    pub(crate) states: Vec<String>,
    lr: f32,
    exp_rate: f32,
    decay_gamma: f32,
    states_value: HashMap<String, f32>,
    name: String,
}
impl Agent {
    pub fn new(name: String, exp_rate: Option<f32>) -> Agent {
        Agent {
            states: vec![],
            lr: 0.2,
            exp_rate: exp_rate.unwrap_or(0.3),
            decay_gamma: 0.9,
            states_value: HashMap::new(),
            name,
        }

    }
    pub fn save(&mut self, filename: String, round: i32) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&filename)
            .unwrap_or(File::create(filename).unwrap());
        let data = Data {
            round,
            data: self.states_value.clone(),
        };
        serde_json::to_writer(&file, &data);

    }
    pub fn try_load(&mut self, filename: String) -> i32 {
        let file = File::open(filename);
        if file.is_err() {
            eprintln!("Failed to load agent");
            return 0;
        }
        let data: Data = serde_json::from_reader(file.unwrap()).unwrap();
        self.states_value = data.data;
        data.round

    }
}
impl AiFunctions for Agent {
    type Position = Move;
    type Board = Chess;
    type Player = Players;

    fn reset(&mut self) {
        self.states = vec![];
    }

    fn choose_action(&mut self, available_positions: Vec<Self::Position>, current_board: Self::Board, _symbol: Self::Player) -> Self::Position {
        let mut rng = thread_rng();
        let mut action = Move::new(King(NULL), (0,0), (0,0));
        if rng.gen_range(0.0..1.0) <= self.exp_rate {
            let index = rng.gen_range(0..available_positions.len());
            action = available_positions[index];
        } else {
            let mut value_max = -999.0;
            let mut value = 0.0;
            let mut next_board = current_board.clone();
            let mut board_hash = compress(next_board.board);
            for play in available_positions {
                next_board = current_board.clone();
                next_board.update(play);
                board_hash = compress(next_board.board);
                value = *self.states_value.get(&board_hash).unwrap_or(&0.0);
                if value >= value_max {
                    value_max = value;
                    action = play;
                }
            }
        }
        action
    }

    fn feed_reward(&mut self, reward: f32) {
        let mut reward_actual = reward;
        for state in self.states.reversed() {
            if self.states_value.get(&state).is_none() {
                self.states_value.insert(state.clone(), 0.0);
            }
            self.states_value.insert(state.clone(), self.states_value.get(&state).unwrap()+(self.lr*(self.decay_gamma*reward_actual - self.states_value.get(&state).unwrap())));
            reward_actual = *self.states_value.get(&state).unwrap();
        }
    }
}
