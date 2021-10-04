#![feature(string_remove_matches)]


mod traits;
mod pieces;
mod moves;
mod agent;
mod chess;

use useful_shit::{GameBoard, input};
use crate::traits::*;
use crate::chess::Chess;

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

impl Coords for (usize, usize) {
    fn to_rank_file(&self) -> String {
        let rank = self.0 as u8+ 65;
        if rank > 72 {
            panic!("Invalid Coord");
        }
        format!("{}{}", rank as char, self.1+1)
    }
}

fn main() {
    let mut game = Chess::new();

    game.reset();
    game.train(1000);
    return;
    loop {
        println!("{}", game);
        println!("Player {:?}", game.get_current_player());
        let mut possible_moves = game.available_positions(game.get_current_player());
        if !game.is_king_safe(game.get_current_player()) {
            possible_moves = game.get_safe_moves(game.get_current_player());
        }
        println!("{}", possible_moves.clone());
        let choice: usize = input("Move #: ");
        let possible_moves = possible_moves.get_inner();
        game.update(possible_moves[choice]);
        let winner = game.check_winner();
        if winner.is_some() {
            println!("WINNER IS: {:?}", winner.unwrap());
            break;
        }

    }

}
//TODO TEST MOVEMENTS