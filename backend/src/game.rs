use serde::Serialize;

use crate::board::{Board, Player, Position};

pub struct Game {
    board: Board,
    status: MatchStatus,
}

#[derive(Clone)]
#[derive(Serialize)]
#[serde(tag = "status")]
pub enum MatchStatus {
    InProgress {player: Player},
    Won {player: Player},
    Draw,
}

impl Default for MatchStatus {
    fn default() -> Self {
        Self::InProgress{player: Player::X}
    }
}

#[derive(Debug)]
pub enum MatchError {
    CellOccupied,
    InvalidMove,
    GameOver,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            status: MatchStatus::default(),
        }
    }
}

impl Game {
    pub fn take_turn(&mut self, position: &Position) -> Result<MatchStatus, MatchError> {
        let current_player = match self.status {
            MatchStatus::InProgress {player} => player,
            _ => return Err(MatchError::GameOver),
        };

        if !position.is_valid() {
            return Err(MatchError::InvalidMove);
        }

        if self.board.get_cell(position).is_some() {
            return Err(MatchError::CellOccupied);
        }
        self.board.set_cell(position, current_player);

        self.status = if let Some(winner) = self.board.winner() {
            MatchStatus::Won {player: winner}
        } else if self.board.is_full() {
            MatchStatus::Draw
        } else {
            MatchStatus::InProgress {player: current_player.other_player()}
        };

        Ok(self.status.clone())
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn status(&self) -> &MatchStatus {
        &self.status
    }

    pub fn valid_moves(&self) -> Vec<Position> {
        self.board.empty_cells()
    }
}
