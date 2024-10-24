use crate::board::{Board, Player, Position};

pub struct Game {
    board: Board,
    current_player: Player,
}

pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

pub enum GameError {
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

    pub fn take_turn(&mut self, position: &Position) -> Result<GameState, GameError> {
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

    pub fn board(&self) -> Board {
        self.board.clone()
    }
}
