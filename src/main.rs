mod board;

use crate::board::{Board, Player, Position};

fn main() {
    let mut board = Board::default();
    println!("{board}");
    let pos = Position {x: 1, y: 1};
    board.set_cell(&pos, Player::X);
    println!("{board}");
    let val = match board.get_cell(&pos) {None => "Empty", Some(player) => match player {Player::X => "X", Player::O => "O"}};
    println!("Value at {pos:?} = {val}");
}
