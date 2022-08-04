
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod betdex_spl_faucet {
    use super::*;
    use anchor_spl::token;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn airdrop(ctx: Context<Airdrop>) -> Result<()> {

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.payer_token_account.to_account_info(),
                    authority: ctx.accounts.token_vault.to_account_info(),
                },
                &[&[ctx.accounts.mint.key().as_ref()]],
            ),
            (100 * 10_u32.pow(ctx.accounts.mint.decimals as u32)) as u64,
        )
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    seeds=[mint.key().as_ref()],
    bump,
    payer=payer,
    token::mint=mint,
    token::authority=token_vault
    )]
    pub token_vault: Account<'info, TokenAccount>,
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
    seeds=[mint.key().as_ref()],
    bump,
    token::mint=mint,
    token::authority=token_vault
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}
