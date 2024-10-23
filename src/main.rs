mod board;

use crate::board::{Board, Player, Position};
use std::io;

struct Game {
    board: Board,
    current_player: Player,
}

enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

enum GameError {
    CellOccupied,
    GameOver,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            current_player: Player::X
        }
    }
}

impl Game {
    fn state(&self) -> GameState {
        if let Some(winner) = self.board.winner() {
            GameState::Won(winner)
        } else if self.board.is_full() {
            GameState::Draw
        } else {
            GameState::InProgress
        }
    }

    fn take_turn(&mut self, position: &Position) -> Result<GameState, GameError> {
        match self.state() {
            GameState::InProgress => (),
            _ => return Err(GameError::GameOver),
        }

        if self.board.get_cell(position).is_some() {
            return Err(GameError::CellOccupied)
        }

        self.board.set_cell(position, self.current_player);
        let state = self.state();

        if matches!(state, GameState::InProgress) {
            self.current_player = self.current_player.other_player();
        }

        Ok(state)
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }
}

fn main() {
    let mut game = Game::default();
    loop {
        println!("{}", game.board);
        println!("Player {}'s turn!", game.current_player());
        let position = loop {
            let mut raw_move = String::new();
            println!("Choose next move: [ x, y ]");

            io::stdin()
                .read_line(&mut raw_move)
                .expect("Failed to read line");

            match raw_move.parse() {
                Ok(pos) => break pos,
                Err(e) => {println!("Invalid input: {}. Please use format 'x,y'", e); continue;},
            }
        };

        match game.take_turn(&position) {
            Ok(GameState::InProgress) => continue,
            Ok(GameState::Won(player)) => {println!("Player {} wins!", player); break;},
            Ok(GameState::Draw) => {println!("Game ended in a draw"); break;},
            Err(GameError::CellOccupied) => println!("Cell already occupied"),
            Err(GameError::GameOver) => println!("Game is already over"),
        }
    }
}
