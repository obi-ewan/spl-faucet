import * as anchor from "@project-serum/anchor";
import {AnchorProvider, BN, Program, web3} from "@project-serum/anchor";
import {SplFaucet} from "../target/types/spl_faucet";
import {PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY} from "@solana/web3.js";
import NodeWallet from "@project-serum/anchor/dist/esm/nodewallet";
import {createAssociatedTokenAccount, createMint, getAccount, mintTo, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import * as assert from "assert";

describe("spl-faucet", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.SplFaucet as Program<SplFaucet>;

    it("Initialize new faucet", async () => {
        const faucet = await initializeFaucet(program, provider)

        const createdConfig = await program.account.faucetConfig.fetch(faucet.faucetConfigPk);
        assert.equal(createdConfig.amount.toNumber(), 100 * 10 ** 9);
        assert.equal(createdConfig.limit.toNumber(), 1000 * 10 ** 9);
    });

    it('Airdrop tokens', async function () {
        const faucet = await initializeFaucet(program, provider)

        const mint = faucet.mint;
        const wallet = provider.wallet as NodeWallet
        const tokenAccount = await createAssociatedTokenAccount(
            provider.connection,
            wallet.payer,
            mint,
            provider.wallet.publicKey
        )

        await mintTo(
            provider.connection,
            wallet.payer,
            mint,
            faucet.tokenVaultPk,
            provider.wallet.publicKey,
            100 * 10 ** 9,
            [],
        );

        await program.methods.airdrop().accounts({
            payer: provider.wallet.publicKey,
            payerTokenAccount: tokenAccount,
            tokenVault: faucet.tokenVaultPk,
            config: faucet.faucetConfigPk,
            mint: faucet.mint,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).rpc();

        const tokenBalance = await provider.connection.getTokenAccountBalance(tokenAccount);
        assert.equal(tokenBalance.value.amount, 100 * 10 ** 9);
    });

    it('Airdrop tokens - hits limit', async function () {
        const faucet = await initializeFaucet(program, provider)

        const mint = faucet.mint;
        const wallet = provider.wallet as NodeWallet
        const tokenAccount = await createAssociatedTokenAccount(
            provider.connection,
            wallet.payer,
            mint,
            provider.wallet.publicKey
        )

        await mintTo(
            provider.connection,
            wallet.payer,
            mint,
            faucet.tokenVaultPk,
            provider.wallet.publicKey,
            100 * 10 ** 9,
            [],
        );

        await mintTo(
            provider.connection,
            wallet.payer,
            mint,
            tokenAccount,
            provider.wallet.publicKey,
            1000 * 10 ** 9,
            [],
        );

        try {
            await program.methods.airdrop().accounts({
                payer: provider.wallet.publicKey,
                payerTokenAccount: tokenAccount,
                tokenVault: faucet.tokenVaultPk,
                config: faucet.faucetConfigPk,
                mint: faucet.mint,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).rpc();
        } catch (e) {
            assert.equal(e.error.errorCode.code, "WalletLimitReached")
        }

        const tokenBalance = await provider.connection.getTokenAccountBalance(tokenAccount);
        assert.equal(tokenBalance.value.amount, 1000 * 10 ** 9);
    });
});


async function initializeFaucet(program: Program<SplFaucet>, provider: AnchorProvider) {
    const mintDecimals = 9;
    const airdropAmount = 100_000_000_000; // 100;
    const limit = 1000_000_000_000; // 1000;

    const wallet = provider.wallet as NodeWallet;
    const mint = await createMint(
        provider.connection,
        wallet.payer,
        wallet.publicKey,
        wallet.publicKey,
        mintDecimals,
    );

    const [tokenVaultPk] = await PublicKey.findProgramAddress(
        [Buffer.from("faucet_token_vault"), mint.toBuffer()], program.programId,
    );

    const [faucetConfigPk] = await PublicKey.findProgramAddress(
        [Buffer.from("faucet_config"), mint.toBuffer()], program.programId,
    );

    await program
        .methods
        .initializeFaucet(new BN(airdropAmount), new BN(limit))
        .accounts({
            tokenVault: tokenVaultPk,
            config: faucetConfigPk,
            payer: provider.wallet.publicKey,
            mint: mint,
            rent: SYSVAR_RENT_PUBKEY,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).rpc();

    return {tokenVaultPk, faucetConfigPk, mint}
}