mod game;
mod board;

use crate::game::{Game, MatchStatus, GameError};
use crate::board::{Board, Position, Player};
use std::io;
use axum::{
    Json,
    extract::{State, Path},
    routing::{get, post},
    Router,
    response::IntoResponse
};
use std::sync::{Arc, Mutex};
use serde::Serialize;

struct AppState {
    game: Mutex<Game>,
}

#[derive(Serialize)]
struct GameState {
    board: Vec<Option<String>>,
    current_player: Option<String>,
    winner: Option<String>,
}

async fn game_state(State(state): State<Arc<AppState>>) -> Json<GameState> {
    let game = state.game.lock().unwrap();
    let state = GameState {
        board: flatten_board(&game.board()),
        current_player: game.current_player().map(|p| p.to_string()),
        winner: game.winner().map(|p| p.to_string())
    };
    Json(state)
}

async fn make_move(
    State(state): State<Arc<AppState>>, 
    Path(move_string): Path<String>) {
        let x = move_string.chars().nth(0).map_or(0, |m| m.to_string().parse::<u8>().expect("Invalid move"));
        let y = move_string.chars().nth(1).map_or(0, |m| m.to_string().parse::<u8>().expect("Invalid move"));
        state.game.lock().unwrap().take_turn(&Position::new(x, y));
}

fn flatten_board(board: &Board) -> Vec<Option<String>> {
    let mut flat = Vec::with_capacity(9);
    for y in 0..3 {
        for x in 0..3 {
            let cell = board.get_cell(&Position::new(x, y));
            flat.push(cell.map(|p| p.to_string()));
        }
    }
    flat
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let state = Arc::new(AppState{ game: Mutex::new(Game::default()) });

    let app = Router::new()
        .route("/", get(game_state))
        .route("/move/:movestr", get(make_move))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


/*
fn main() {
    let mut game = Game::default();
    loop {
        println!("{}", game.board());
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
*/