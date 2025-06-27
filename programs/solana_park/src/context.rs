

// context.rs
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::Token2022, token_interface::*};
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 128, seeds = [b"game"], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK:
    pub server: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateParkToken<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"park_mint"], bump)]
    pub park_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    pub token_2022_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct CreateRoom<'info> {
    #[account(init, payer = host, space = 8 + 256)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub host: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinRoom<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct PayAndStart<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub gor_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub park_mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_2022_program: Program<'info, Token2022>,
}
