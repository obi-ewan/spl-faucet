use anchor_lang::prelude::*;

#[account]
pub struct FaucetConfig {
    pub amount: u64,    // number of tokens airdropped
    pub limit: u64,      // max number of tokens a wallet can have present
    pub mint: Pubkey,
}
