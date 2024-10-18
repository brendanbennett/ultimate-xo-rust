

#[derive(Debug)]
struct Board {
    cells: [u8; 9]
}

impl Board {
    fn set_cell(&mut self, position: &Position, value: u8) {
        self.cells[(position.y * 3 + position.x) as usize] = value
    }

    fn get_cell(&self, position: &Position) -> u8 {
        self.cells[(position.y * 3 + position.x) as usize]
    }
}

#[derive(Debug)]
struct Position{
    x: u8,
    y: u8,
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
