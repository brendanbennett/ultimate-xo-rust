mod game;
mod board;

use crate::game::{Game, GameState, GameError};
use std::io;
use axum::{
    extract::State,
    routing::get,
    Router,
    response::IntoResponse
};
use std::sync::{Arc, Mutex};

struct AppState {
    game: Mutex<Game>,
}

async fn board(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let game = state.game.lock().unwrap();
    format!("{}", game.board()).into_response()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let state = Arc::new(AppState{ game: Mutex::new(Game::default()) });

    let app = Router::new().route("/", get(board)).with_state(state);

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