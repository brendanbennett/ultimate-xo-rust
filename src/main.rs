use std::fmt;

#[derive(Debug)]
struct Board {
    bitboards: [u16; 3] // X: player 0, O: player 1, and last move
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
enum Player {
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

impl Board {
    fn set_cell(&mut self, position: &Position, player: Player) {
        // Need to update next move
        let mask = (1 as u16) << (position.y * 3 + position.x);
        self.bitboards[player as usize] ^= mask;
    }

    fn get_cell(&self, position: &Position) -> Option<Player> {
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

    fn winner(&self) -> Option<Player> {
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
        Self { bitboards: [0, 0, 0]}
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
struct Position{
    x: u8,
    y: u8,
}

const WINNING: [u16; 8] = [
    73, 73 << 1, 73 << 2,
    7, 7 << 3, 7 << 6,
    273, 84,
];

fn main() {
    let mut board = Board::default();
    println!("{board}");
    let pos = Position {x: 1, y: 1};
    board.set_cell(&pos, Player::X);
    println!("{board}");
    let val = match board.get_cell(&pos) {None => "Empty", Some(player) => match player {Player::X => "X", Player::O => "O"}};
    println!("Value at {pos:?} = {val}");
    
    assert_eq!(Board { bitboards: [WINNING[0], 0, 0] }.winner().unwrap(), Player::X);
    assert_eq!(Board { bitboards: [0, WINNING[4], 0] }.winner().unwrap(), Player::O);
    assert_eq!(Board { bitboards: [6, 16, 0] }.winner(), None)
}
