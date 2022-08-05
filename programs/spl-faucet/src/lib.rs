
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spl_faucet {
    use super::*;
    use anchor_spl::token;

    pub fn initialize_faucet(ctx: Context<Initialize>, airdrop_amount: u64, token_limit: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.amount = airdrop_amount;
        config.limit = token_limit;
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {
        require!(
            ctx.accounts.payer_token_account.amount < ctx.accounts.config.limit,
            FaucetError::WalletLimitReached
        );

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.payer_token_account.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                &[&[
                    "faucet_token_vault".as_ref(), ctx.accounts.mint.key().as_ref(),
                    &[*ctx.bumps.get("token_vault").unwrap()]
                ]],
            ),
            ctx.accounts.config.amount,
        )
    }
}

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
        space= 8 + 8 + 8
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
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"faucet_token_vault".as_ref(), mint.key().as_ref()],
        bump,
        token::mint=mint,
        token::authority=token_vault
    )]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"faucet_config".as_ref(), mint.key().as_ref()],
        bump
    )]
    pub config: Account<'info, FaucetConfig>,
    pub mint: Account<'info, Mint>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct FaucetConfig {
    pub amount: u64,    // number of tokens airdropped
    pub limit: u64      // max number of tokens a wallet can have present
}

#[error_code]
pub enum FaucetError {
    #[msg("Wallet has reached token limit for this faucet.")]
    WalletLimitReached
}