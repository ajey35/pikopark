// error.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Room is full.")]
    RoomFull,
    #[msg("Not enough players.")]
    NotEnoughPlayers,
    #[msg("Invalid room state.")]
    InvalidRoomState,
}