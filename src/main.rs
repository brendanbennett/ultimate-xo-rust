#[derive(Debug)]
struct Board {
    bitboard: [u16; 3] // X: player 0, O: player 1, and last move
}

impl Board {
    fn bitboard_idx_from_player(&self, player: Player) -> u8{
        match player {
            Player::X => 0,
            Player::Y => 1,
        }
    }

    fn set_cell(&mut self, position: &Position, player: Player) {
        // Need to update next move
        let idx = self.bitboard_idx_from_player(player);
        let mask = (1 as u16) << (position.y * 3 + position.x);
        bitboard[idx] ^= mask;
    }

    fn get_cell(&self, position: &Position) -> u8 {
        // Need to decide what this does
        let mask = (1 as u16) << (position.y * 3 + position.x);

    }
}

#[derive(Debug)]
struct Position{ // Add None?
    x: u8,
    y: u8,
}

enum Player {
    X,
    O,
}

fn build_board() -> Board {
    Board {
        cells: [0; 9]
    }
}

fn main() {
    let mut board = build_board();
    println!("{board:?}");
    let pos = Position {x: 1, y: 1};
    board.set_cell(&pos, 1);
    println!("{board:?}");
    let val = board.get_cell(&pos);
    println!("Value at {pos:?} = {val}");
}
