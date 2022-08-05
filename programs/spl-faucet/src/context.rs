use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::FaucetConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[b"faucet_token_vault".as_ref(), mint.key().as_ref()],
        bump,
        payer=payer,
        token::mint=mint,
        token::authority=token_vault
    )]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds=[b"faucet_config".as_ref(), mint.key().as_ref()],
        bump,
        payer=payer,
        space= 8 + 8 + 8 + 32
    )]
    pub config: Account<'info, FaucetConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address=system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = config.mint,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"faucet_token_vault".as_ref(), mint.key().as_ref()],
        bump,
        token::mint=mint,
        token::authority=token_vault
    )]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(seeds=[b"faucet_config".as_ref(), mint.key().as_ref()], bump)]
    pub config: Account<'info, FaucetConfig>,
    pub mint: Account<'info, Mint>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}