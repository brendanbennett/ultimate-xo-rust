use crate::board::{Board, Player, Position};

pub struct Game {
    board: Board,
    status: MatchStatus,
}

#[derive(Clone)]
pub enum MatchStatus {
    InProgress(Player),
    Won(Player),
    Draw,
}

impl Default for MatchStatus {
    fn default() -> Self {
        Self::InProgress(Player::X)
    }
}

pub enum GameError {
    CellOccupied,
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
    pub fn take_turn(&mut self, position: &Position) -> Result<MatchStatus, GameError> {
        let current_player = match self.status {
            MatchStatus::InProgress(player) => player,
            _ => return Err(GameError::GameOver),
        };

        if self.board.get_cell(position).is_some() {
            return Err(GameError::CellOccupied);
        }
        self.board.set_cell(position, current_player);

        self.status = if let Some(winner) = self.board.winner() {
            MatchStatus::Won(winner)
        } else if self.board.is_full() {
            MatchStatus::Draw
        } else {
            MatchStatus::InProgress(current_player.other_player())
        };

        Ok(self.status.clone())
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Option<Player> {
        match self.status {
            MatchStatus::InProgress(current_player) => Some(current_player),
            _ => None,
        }
    }

    pub fn status(&self) -> &MatchStatus {
        &self.status
    }

    pub fn winner(&self) -> Option<Player> {
        match self.status {
            MatchStatus::Won(winner) => Some(winner),
            _ => None,
        }
    }
}
