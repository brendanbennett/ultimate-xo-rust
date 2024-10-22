use std::fmt;

#[derive(Debug)]
pub struct Board {
    bitboards: [u16; 2] // X: player 0, O: player 1, and last move
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X = 0,
    O = 1,
}

impl Player {
    const PLAYERS: [Self; 2] = [Player::X, Player::O];
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
        self.bitboards[(1 - player as u8) as usize] &= !mask;
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
                if self.bitboards[player.clone() as usize] == win_case {
                    return Some(player)
                }
            }
        }
        None
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
                let cell = self.get_cell(&Position {x: x as u8, y: y as u8});
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

#[derive(Debug)]
pub struct Position{
    pub x: u8,
    pub y: u8,
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
        b.set_cell(&Position {x: 1, y: 1}, Player::X);
        assert_eq!(b.bitboards[Player::X as usize], 16);
    }

    #[test]
    fn get_cell() {
        let b = Board { bitboards: [1, 0] };
        assert_eq!(b.get_cell(&Position { x: 0, y: 0 }), Some(Player::X));
    }

    #[test]
    fn set_get_cell() {
        let mut b = Board::default();
        let player = Player::X;
        let pos = Position {x: 1, y: 1};
        b.set_cell(&pos, player);
        assert_eq!(b.get_cell(&pos), Some(player));
    }

    #[test]
    fn win() {
        let mut b = Board::default();
        b.set_cell(&Position {x: 0, y: 1}, Player::X);
        b.set_cell(&Position {x: 1, y: 0}, Player::X);
        b.set_cell(&Position {x: 2, y: 2}, Player::X);
        b.set_cell(&Position {x: 2, y: 0}, Player::O);
        b.set_cell(&Position {x: 1, y: 1}, Player::O);
        b.set_cell(&Position {x: 0, y: 2}, Player::O);
        assert_eq!(b.winner(), Some(Player::O));
    }

    #[test]
    fn reset_other_player() {
        let mut b = Board::default();
        let pos = Position {x: 1, y: 1};
        b.set_cell(&pos, Player::X);
        b.set_cell(&pos, Player::O);
        assert_eq!(b.get_cell(&pos), Some(Player::O));
    }
}
