use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{
        initialize_mint, mint_to, set_authority, transfer_checked,
        InitializeMint, MintTo, SetAuthority, Token2022, TransferChecked,
    },
    token_interface::{Mint, TokenAccount},
};
use spl_token_metadata_interface::state::{Field, TokenMetadata};

mod context;
mod state;
mod error;

use context::*;
use state::*;
use error::*;

declare_id!("PARKv1eW1uZLHJ7R5S5A1cHZb9VrTx5FJ4tJ7xZ8d7F");

#[program]
pub mod solana_park {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        game_state.admin = ctx.accounts.admin.key();
        game_state.server_wallet = ctx.accounts.server.key();
        game_state.bump = bump; // Store PDA bump
        Ok(())
    }

    pub fn create_park_token(ctx: Context<CreateParkToken>) -> Result<()> {
        // Initialize mint
        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_2022_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.park_mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            6,
            &ctx.accounts.game_state.key(),
            Some(&ctx.accounts.game_state.key()),
        )?;

        // Set up metadata
        let metadata = TokenMetadata {
            name: "PARK Token".to_string(),
            symbol: "PARK".to_string(),
            uri: "https://metadata.solanapark.io/park.json".to_string(),
            update_authority: Some(ctx.accounts.game_state.key()),
            mint: ctx.accounts.park_mint.key(),
            additional_metadata: vec![
                (Field::Description, "Reward token for Solana Park game"),
                (Field::Image, "https://metadata.solanapark.io/park.png"),
            ],
        };

        // Initialize metadata extension
        spl_token_2022::extension::metadata::initialize(
            &ctx.accounts.park_mint.to_account_info(),
            ctx.accounts.token_2022_program.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            metadata,
        )?;

        Ok(())
    }

    pub fn create_room(ctx: Context<CreateRoom>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        room.host = ctx.accounts.host.key();
        room.created_at = Clock::get()?.unix_timestamp;
        room.expires_at = room.created_at + 1800;
        room.status = RoomStatus::Waiting;
        room.players = Vec::new(); // Initialize players vector
        room.started_at = 0; // Initialize
        room.ended_at = 0; // Initialize
        Ok(())
    }

    pub fn join_room(ctx: Context<JoinRoom>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        require!(room.players.len() < 4, GameError::RoomFull);
        room.players.push(ctx.accounts.player.key());
        Ok(())
    }

    pub fn pay_and_start(ctx: Context<PayAndStart>, maps: Vec<u8>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        require!(room.status == RoomStatus::Waiting, GameError::InvalidRoomState);
        require!(room.players.len() >= 2, GameError::NotEnoughPlayers);

        // Transfer GOR tokens
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    mint: ctx.accounts.gor_mint.to_account_info(),
                    authority: ctx.accounts.host.to_account_info(),
                },
            ),
            1_000_000, // 1 GOR
            6,
        )?;

        // Update room state
        room.status = RoomStatus::Active;
        room.selected_maps = maps;
        room.started_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn end_game(ctx: Context<EndGame>, player: Pubkey, tokens: u64) -> Result<()> {
        let room = &mut ctx.accounts.room;
        require!(
            room.status == RoomStatus::Active || room.status == RoomStatus::Completed,
            GameError::InvalidRoomState
        );
        
        // Mint PARK tokens to player
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_2022_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.park_mint.to_account_info(),
                    to: ctx.accounts.player_ata.to_account_info(),
                    authority: ctx.accounts.game_state.to_account_info(),
                },
                &[&[b"game", &[ctx.accounts.game_state.bump]]],
            ),
            tokens * 1_000_000, // Scale to decimals
        )?;

        // Update room status if first completion
        if room.status != RoomStatus::Completed {
            room.status = RoomStatus::Completed;
            room.ended_at = Clock::get()?.unix_timestamp;
        }

        Ok(())
    }
}