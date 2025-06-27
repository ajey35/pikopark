
// state.rs
use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub admin: Pubkey,
    pub server_wallet: Pubkey,
    pub bump: u8,
}

#[account]
pub struct Room {
    pub host: Pubkey,
    pub players: Vec<Pubkey>,
    pub status: RoomStatus,
    pub selected_maps: Vec<u8>,
    pub created_at: i64,
    pub expires_at: i64,
    pub started_at: i64,
    pub ended_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RoomStatus {
    Waiting,
    Active,
    Completed,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PlayerScore {
    pub player: Pubkey,
    pub tokens: u64,
}
