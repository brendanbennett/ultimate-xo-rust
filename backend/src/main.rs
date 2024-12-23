mod game;
mod board;

use crate::game::{Game, MatchStatus, MatchError};
use crate::board::{Board, Position};
use axum::{
    Json,
    extract::{State, Path},
    routing::{get, post},
    Router,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use axum_macros::debug_handler;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;

struct AppState {
    game: Mutex<HashMap<String, Game>>,
}

#[derive(Serialize)]
enum GameError {
    GameNotFound,
}

#[derive(Serialize)]
struct GameState {
    board: Vec<Option<String>>,
    valid_moves: Vec<bool>,
    status: MatchStatus,
}

impl IntoResponse for MatchError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            MatchError::GameOver => (StatusCode::BAD_REQUEST, "Game is over"),
            MatchError::InvalidMove => (StatusCode::BAD_REQUEST, "Invalid Move"),
            MatchError::CellOccupied => (StatusCode::BAD_REQUEST, "Cell occupied"),
        };

        (status, message).into_response()
    }
}

async fn game_state(State(state): State<Arc<AppState>>) -> Json<GameState> {
    let game = state.game.lock().unwrap();
    let state = GameState {
        board: flatten_board(&game.board()),
        valid_moves: flatten_positions(game.valid_moves()),
        status: game.status().clone(),
    };
    Json(state)
}

#[debug_handler]
async fn make_move(
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>, 
    Json(move_position): Json<Vec<u32>>) -> Result<Json<GameState>, MatchError> {
    println!("Move {:?}", move_position);

    let mut game = if let Some(game) = state.game.lock().unwrap().get(&game_id) {
        game
    } else {
        return Err(GameError::GameNotFound)
    };
    let move_position = Position::from_vec(move_position)
        .map_err(|_| MatchError::InvalidMove)?;

    let match_status = game.take_turn(&move_position)?;

    let state = GameState {
        board: flatten_board(&game.board()),
        valid_moves: flatten_positions(game.valid_moves()),
        status: match_status,
    };
    Ok(Json(state))
}

async fn new_game(State(state): State<Arc<AppState>>) -> String {
    let mut game = state.game.lock().unwrap();
    *game = Game::default();


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

fn flatten_positions(positions: Vec<Position>) -> Vec<bool> {
    let mut flat = [false; 9];
    for pos in positions {
        flat[(pos.x + pos.y * 3) as usize] = true
    }
    flat.to_vec()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let state = Arc::new(AppState{ game: Mutex::new(Game::default()) });

    let app = Router::new()
        .route("/api/game/:id", get(game_state))
        .route("/api/game/:id/move", post(make_move))
        .route("/api/game/new", get(new_game))
        .route("/", get(|| async { "Hello, World!" }))
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