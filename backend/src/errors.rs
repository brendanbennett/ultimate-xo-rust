use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::Serialize;
use crate::game::MatchError;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    GameError(GameError),
    MatchError(MatchError),
    InvalidRequest(String),
}

#[derive(Debug, Serialize)]
pub enum GameError {
    GameNotFound,
    GameFull,
    InvalidGameId,
    InternalError,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "detail")]
pub enum ErrorResponse {
    GameError { code: &'static str, message: String },
    MatchError { code: &'static str, message: String },
    InvalidRequest { message: String },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_response) = match self {
            ApiError::GameError(game_error) => match game_error {
                GameError::GameNotFound => (
                    StatusCode::NOT_FOUND,
                    ErrorResponse::GameError {
                        code: "GAME_NOT_FOUND",
                        message: "The requested game does not exist".to_string(),
                    },
                ),
                GameError::GameFull => (
                    StatusCode::CONFLICT,
                    ErrorResponse::GameError {
                        code: "GAME_FULL",
                        message: "This game is already full".to_string(),
                    },
                ),
                GameError::InvalidGameId => (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::GameError {
                        code: "INVALID_GAME_ID",
                        message: "The provided game ID is invalid".to_string(),
                    },
                ),
                GameError::InternalError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse::GameError {
                        code: "INTERNAL_ERROR",
                        message: "An internal server error occurred".to_string(),
                    },
                ),
            },
            ApiError::MatchError(match_error) => match match_error {
                MatchError::GameOver => (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::MatchError {
                        code: "GAME_OVER",
                        message: "This game has already ended".to_string(),
                    },
                ),
                MatchError::InvalidMove => (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::MatchError {
                        code: "INVALID_MOVE",
                        message: "The requested move is invalid".to_string(),
                    },
                ),
                MatchError::CellOccupied => (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse::MatchError {
                        code: "CELL_OCCUPIED",
                        message: "The selected cell is already occupied".to_string(),
                    },
                ),
            },
            ApiError::InvalidRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::InvalidRequest { message: msg },
            ),
        };

        (status, Json(error_response)).into_response()
    }
}

// Implement From traits for convenient error conversion
impl From<GameError> for ApiError {
    fn from(error: GameError) -> Self {
        ApiError::GameError(error)
    }
}

impl From<MatchError> for ApiError {
    fn from(error: MatchError) -> Self {
        ApiError::MatchError(error)
    }
}
