#![feature(string_remove_matches)]

mod agent;
mod chess;
mod moves;
mod pieces;
mod traits;

use crate::chess::Chess;
use useful_shit::Players::{BLACK, NULL};
use useful_shit::{compress, input};

fn main() {
    let mut game = Chess::new();

    game.reset();
    #[cfg(feature = "train")]
    {
        game.train(1000);
        return;
    }

    #[cfg(feature = "play")]
    {
        game.play();
        return;
    }
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
        game.update_game(possible_moves[choice]);
        let winner = game.check_winner();
        println!("Winner would be {:?}", winner.unwrap());
    }
}
//TODO TEST MOVEMENTS
