pub mod context;
pub mod state;

use anchor_lang::prelude::*;
use crate::context::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spl_faucet {
    use super::*;
    use anchor_spl::token;

    pub fn initialize_faucet(ctx: Context<Initialize>, airdrop_amount: u64, token_limit: u64) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.amount = airdrop_amount;
        config.limit = token_limit;
        config.mint = ctx.accounts.mint.key();
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

#[error_code]
pub enum FaucetError {
    #[msg("Wallet has reached token limit for this faucet.")]
    WalletLimitReached
}