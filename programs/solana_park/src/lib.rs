// lib.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{
        initialize_mint, mint_to, set_authority, transfer_checked,
        InitializeMint, MintTo, SetAuthority, Token2022, TransferChecked,
    },
    token_interface::{Mint, TokenAccount},
};
use spl_associated_token_account::get_associated_token_address_with_program_id;

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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        game_state.admin = ctx.accounts.admin.key();
        game_state.server_wallet = ctx.accounts.server.key();
        game_state.bump = *ctx.bumps.get("game_state").unwrap();
        Ok(())
    }

    pub fn create_park_token(ctx: Context<CreateParkToken>) -> Result<()> {
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

        set_authority(
            CpiContext::new_with_signer(
                ctx.accounts.token_2022_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.park_mint.to_account_info(),
                    current_authority: ctx.accounts.game_state.to_account_info(),
                },
                &[&[b"game", &[ctx.bumps.get("game_state").unwrap().clone()]]],
            ),
            spl_token_2022::instruction::AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }

    pub fn create_room(ctx: Context<CreateRoom>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        room.host = ctx.accounts.host.key();
        room.created_at = Clock::get()?.unix_timestamp;
        room.expires_at = room.created_at + 1800;
        room.status = RoomStatus::Waiting;
        room.players = vec![];
        room.selected_maps = vec![];
        room.started_at = 0;
        room.ended_at = 0;
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
        require!(room.players.len() >= 2, GameError::NotEnoughPlayers);

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
            1_000_000,
            6,
        )?;

        room.status = RoomStatus::Active;
        room.selected_maps = maps;
        room.started_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn end_game(ctx: Context<EndGame>, scores: Vec<PlayerScore>) -> Result<()> {
        let room = &mut ctx.accounts.room;
        require!(room.status == RoomStatus::Active, GameError::InvalidRoomState);
        room.status = RoomStatus::Completed;
        room.ended_at = Clock::get()?.unix_timestamp;

        for score in scores.iter() {
            let ata = get_associated_token_address_with_program_id(
                &score.player,
                &ctx.accounts.park_mint.key(),
                &anchor_spl::token_2022::ID,
            );

            let cpi_accounts = MintTo {
                mint: ctx.accounts.park_mint.to_account_info(),
                to: AccountInfo::new(&ata, false),
                authority: ctx.accounts.game_state.to_account_info(),
            };

            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_2022_program.to_account_info(),
                    cpi_accounts,
                    &[&[b"game", &[ctx.accounts.game_state.bump]]],
                ),
                score.tokens * 1_000_000,
            )?;
        }

        Ok(())
    }
}