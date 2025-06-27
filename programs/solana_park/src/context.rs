use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::Token2022, token_interface::*};
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32 + 1, seeds = [b"game"], bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: Server wallet (safe)
    pub server: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateParkToken<'info> {
    #[account(mut, seeds = [b"game"], bump = game_state.bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [b"park_mint"],
        bump,
        space = 400, // Sufficient for metadata
        owner = token_2022_program.key(),
    )]
    /// CHECK: Initialized by token program
    pub park_mint: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_2022_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct CreateRoom<'info> {
    #[account(init, payer = host, space = 8 + 32 + 4 + (32*4) + 1 + 4 + 10 + 8*4)]
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
    pub from: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub to: InterfaceAccount<'info, TokenAccount>,
    pub gor_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub room: Account<'info, Room>,
    #[account(mut, seeds = [b"game"], bump = game_state.bump)]
    pub game_state: Account<'info, GameState>,
    #[account(mut, seeds = [b"park_mint"], bump)]
    /// CHECK: Initialized by token program
    pub park_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub player_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_2022_program: Program<'info, Token2022>,
}