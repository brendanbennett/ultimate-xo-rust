use std::fmt;
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Board {
    bitboards: [u16; 2] // X: player 0, O: player 1, and last move
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Player {
    X = 0,
    O = 1,
}

impl Player {
    const PLAYERS: [Self; 2] = [Player::X, Player::O];

    pub fn other_player(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Player::X => "X",
            Player::O => "O",
        })
    }
}

const WINNING: [u16; 8] = [
    73, 73 << 1, 73 << 2,
    7, 7 << 3, 7 << 6,
    273, 84,
];

impl Board {
    pub fn set_cell(&mut self, position: &Position, player: Player) {
        // Need to update next move
        let mask = (1 as u16) << (position.y * 3 + position.x);
        self.bitboards[player as usize] |= mask;
        self.bitboards[player.other_player() as usize] &= !mask;
    }

    pub fn get_cell(&self, position: &Position) -> Option<Player> {
        // Need to decide what this does
        let offset = position.y * 3 + position.x;
        let mask = (1 as u16) << offset;
        let is_player_x = mask & self.bitboards[Player::X as usize] != 0;
        let is_player_o = mask & self.bitboards[Player::O as usize] != 0;
        // println!("[{}, {}], board X: {:#018b}, mask: {:#018b}", bits[0], bits[1], self.bitboards[1], mask);
        if is_player_x && is_player_o {
            panic!("{position:?} set for both X and O")
        } else if is_player_o {
            Some(Player::O)
        } else if is_player_x {
            Some(Player::X)
        } else {
            None
        }
    }

    pub fn winner(&self) -> Option<Player> {
        for player in Player::PLAYERS {
            for win_case in WINNING {
                if !self.bitboards[player.clone() as usize] & win_case == 0 {
                    return Some(player)
                }
            }
        }
        None
    }

    fn count_player(&self, player: Player) -> u32 {
        (self.bitboards[player as usize] << 7).count_ones()
    }

    pub fn is_full(&self) -> bool {
        // Assumes board is valid
        self.count_player(Player::X) + self.count_player(Player::O) == 9
    }

    pub fn available_moves(&self) -> Vec<Position> {
        let valid_bits = !(self.bitboards[0] | self.bitboards[1]);
        let mut available_moves: Vec<Position> = Vec::new();
        for i in 0..9 {
            if 1 & (valid_bits >> i) == 1 {
                available_moves.push(Position::new(i % 3, i / 3))
            }
        }
        available_moves
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { bitboards: [0, 0]}
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                let cell = self.get_cell(&Position::new(x as u8, y as u8));
                write!(f, " {} ", cell.map_or(" ".to_string(), |p| p.to_string()))?;
                if x == 0 || x == 1 {
                    write!(f, "|")?;
                }
            }
            if y == 0 || y == 1 {
                write!(f, "\n{}", "-".repeat(10))?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Position{
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self {x: x, y: y}
    }

    pub fn from_vec(vec: Vec<u32>) -> Result<Self, String> {
        if vec.len() != 2 {
            return Err("Incorrect number of coordinates".to_string())
        }
        Ok(Self {x: vec[0] as u8, y: vec[1] as u8})
    }

    pub fn is_valid(&self) -> bool {
        if self.x > 2 || self.y > 2 {
            return false;
        }
        true
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(",").map(|s| s.trim()).collect();
        if split.len() != 2 {
            return Err(format!("Move requires 2 arguments, received {} with {}", s, split.len()).to_string());
        }
        
        let x = split[0].parse::<u8>()
            .map_err(|_| "Invalid x coordinate".to_string())?;
        let y = split[1].parse::<u8>()
            .map_err(|_| "Invalid y coordinate".to_string())?;

        Ok(Position::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_default() {
        let b = Board::default();
        assert_eq!(b.bitboards, [0,0]);
    }

    #[test]
    fn set_cell() {
        let mut b = Board::default();
        b.set_cell(&Position::new(1, 1), Player::X);
        assert_eq!(b.bitboards[Player::X as usize], 16);
    }

    #[test]
    fn get_cell() {
        let b = Board { bitboards: [1, 0] };
        assert_eq!(b.get_cell(&Position::new(0, 0)), Some(Player::X));
    }

    #[test]
    fn set_get_cell() {
        let mut b = Board::default();
        let player = Player::X;
        let pos = Position::new(1, 1);
        b.set_cell(&pos, player);
        assert_eq!(b.get_cell(&pos), Some(player));
    }

    #[test]
    fn win() {
        let mut b = Board::default();
        b.set_cell(&Position::new(0, 0), Player::X);
        b.set_cell(&Position::new(2, 0), Player::X);
        b.set_cell(&Position::new(1, 1), Player::X);
        b.set_cell(&Position::new(1, 2), Player::X);
        b.set_cell(&Position::new(0, 2), Player::X);
        b.set_cell(&Position::new(1, 0), Player::O);
        b.set_cell(&Position::new(0, 1), Player::O);
        b.set_cell(&Position::new(2, 1), Player::O);
        b.set_cell(&Position::new(2, 2), Player::O);
        assert_eq!(b.winner(), Some(Player::X));
    }

    #[test]
    fn reset_other_player() {
        let mut b = Board::default();
        let pos = Position::new(1, 1);
        b.set_cell(&pos, Player::X);
        b.set_cell(&pos, Player::O);
        assert_eq!(b.get_cell(&pos), Some(Player::O));
    }

    #[test]
    fn test_count() {
        let mut b = Board::default();
        b.set_cell(&Position::new(0, 1), Player::X);
        b.set_cell(&Position::new(1, 0), Player::X);
        b.set_cell(&Position::new(2, 2), Player::X);
        b.set_cell(&Position::new(2, 0), Player::O);
        b.set_cell(&Position::new(1, 1), Player::O);
        b.set_cell(&Position::new(0, 2), Player::O);
        assert_eq!(b.count_player(Player::X), 3);
        assert_eq!(b.count_player(Player::O), 3);
    }

    #[test]
    fn test_valid_moves() {
        let mut b = Board::default();
        b.set_cell(&Position::new(0, 1), Player::X);
        b.set_cell(&Position::new(1, 0), Player::X);
        b.set_cell(&Position::new(2, 2), Player::X);
        b.set_cell(&Position::new(2, 0), Player::O);
        b.set_cell(&Position::new(1, 1), Player::O);
        b.set_cell(&Position::new(0, 2), Player::O);
        assert!(b.available_moves().contains(&Position::new(0, 0)));
        assert_eq!(b.available_moves().len(), 3);
    }

    #[test]
    fn test_position_parse() {
        let pos1: Position = "1,2".parse().unwrap();
        assert!("13eq,3".parse::<Position>().is_err());
    }
}
