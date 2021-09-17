#![feature(string_remove_matches)]


mod traits;
mod pieces;
mod moves;
mod agent;
mod chess;

use useful_shit::{GameBoard, input, Players};
use std::collections::HashMap;
use std::fs::File;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use useful_shit::Players::{WHITE, BLACK, NULL};
use pieces::Pieces;
use crate::traits::*;
use crate::chess::Chess;
use crate::moves::Move;

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
    loop {
        println!("{}", game);
        println!("Player {:?}", game.get_current_player());
        println!("{}", game.available_positions(game.get_current_player()));
        let possible_moves = game.available_positions(game.get_current_player());
        let possible_moves = possible_moves.get_inner();
        let choice: usize = input("Move #: ");

        game.update(possible_moves[choice]);
    }

}

//TODO OTHER PIECE MOVEMENTS
//TODO IMPLEMENT AI