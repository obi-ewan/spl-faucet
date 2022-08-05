# Solana SPL Token Faucet

A simple solana faucet implementation using Anchor.


Create a new faucet:
```typescript
const mintDecimals = 9; // decimals used by airdrop token
const airdropAmount = 100 * 10 ** mintDecimals; // how many tokens to be airdropped per interaction
const limit = 1000 * 10 ** mintDecimals; // max tokens in airdrop-requester's token account

// find PDA for faucet token vault
const [tokenVaultPk] = await PublicKey.findProgramAddress(
    [Buffer.from("faucet_token_vault"), mint.publicKey.toBuffer()], 
    program.programId,
);

// find PDA for faucet config
const [faucetConfigPk] = await PublicKey.findProgramAddress(
    [Buffer.from("faucet_config"), mint.publicKey.toBuffer()], 
    program.programId,
);

await program
    .methods
    .initializeFaucet(new BN(airdropAmount), new BN(limit))
    .accounts({
        tokenVault: tokenVaultPda,
        config: faucetConfigPda,
        payer: provider.wallet.publicKey,
        mint: mint,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();

```

Airdrop from faucet:

```typescript
// create associated spl token account
const tokenAccount = await createAssociatedTokenAccount(
    provider.connection,
    wallet.payer,
    mint,
    provider.wallet.publicKey
);

// or use existing
const tokenAccount = await provider.connection.getTokenAccountsByOwner(
    provider.wallet.publicKey,
    {
        mint: mint
    }
);

await program
    .methods
    .airdrop()
    .accounts({
        payer: provider.wallet.publicKey,
        payerTokenAccount: tokenAccount, // airdrop destination
        tokenVault: tokenVaultPda,
        config: faucetConfigPda,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();

```
