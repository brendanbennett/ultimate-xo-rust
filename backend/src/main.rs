mod game;
mod board;
mod errors;

use uuid::Uuid;
use crate::errors::{ApiError, GameError};
use crate::game::{Game, MatchStatus, MatchError};
use crate::board::{Board, Position};
use axum::{
    Json,
    extract::{State, Path},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::Serialize;

struct AppState {
    games: Mutex<HashMap<String, Game>>,
}

#[derive(Serialize)]
struct GameState {
    board: Vec<Option<String>>,
    valid_moves: Vec<bool>,
    status: MatchStatus,
}

fn generate_game_id() -> String {
    Uuid::new_v4().simple().encode_upper(&mut Uuid::encode_buffer()).to_string()
}

async fn game_state(
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>
) -> Result<Json<GameState>, ApiError> {
    let games = state.games.lock().map_err(|_| GameError::InternalError)?;
    let game = games.get(&game_id).ok_or(GameError::GameNotFound)?;
    let state = GameState {
        board: flatten_board(&game.board()),
        valid_moves: flatten_positions(game.valid_moves()),
        status: game.status().clone(),
    };
    Ok(Json(state))
}

#[debug_handler]
async fn make_move(
    Path(game_id): Path<String>,
    State(state): State<Arc<AppState>>, 
    Json(move_position): Json<Vec<u32>>
) -> Result<Json<GameState>, ApiError> {
    println!("Move {:?}", move_position);

    let mut games = state.games.lock().map_err(|_| GameError::InternalError)?;

    let game = games.get_mut(&game_id).ok_or(GameError::GameNotFound)?;

    let move_position = Position::from_vec(move_position)
        .map_err(|_| ApiError::InvalidRequest("Invalid move format".to_string()))?;

    let match_status = game.take_turn(&move_position)?;

    let state = GameState {
        board: flatten_board(&game.board()),
        valid_moves: flatten_positions(game.valid_moves()),
        status: match_status,
    };
    Ok(Json(state))
}

async fn new_game(State(state): State<Arc<AppState>>) -> Result<Json<String>, ApiError> {
    let mut games = state.games.lock().map_err(|_| GameError::InternalError)?;

    let game_id = generate_game_id();
    games.insert(game_id.clone(), Game::default());

    Ok(Json(game_id))
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
    let state = Arc::new(AppState{ games: Mutex::new(HashMap::new()) });

    let app = Router::new()
        .route("/api/game/:id", get(game_state))
        .route("/api/game/:id/move", post(make_move))
        .route("/api/game/new", get(new_game))
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:42069").await.unwrap();
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