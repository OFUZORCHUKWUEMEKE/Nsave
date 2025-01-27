import * as anchor from '@coral-xyz/anchor';
import type { Program } from '@coral-xyz/anchor';
import type NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    createMint,
    getAssociatedTokenAddressSync,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    getAssociatedTokenAddress,
    createAssociatedTokenAccount
} from '@solana/spl-token';
import { Nsave } from '../target/types/nsave';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';
import { assert, expect } from 'chai';

describe('Nsave', () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Nsave as Program<Nsave>;

    let mint: PublicKey;
    let tokenVaultAccount: PublicKey;
    let savingsAccount: PublicKey;
    let protocolAccount: PublicKey;
    let userAta: PublicKey;
    const wallet = provider.wallet as NodeWallet;

    before(async () => {
        mint = await createMint(
            provider.connection,
            wallet.payer,
            provider.wallet.publicKey,
            null,
            6 // Decimals for USDC
        );

        // Derive the protocol account address
        [protocolAccount] = await PublicKey.findProgramAddressSync(
            [Buffer.from("protocol"), provider.wallet.publicKey.toBuffer()],
            program.programId
        );

        // Derive the savings account address
        const name = "Test Savings";
        const description = "This is a test savings account";
        [savingsAccount] = await PublicKey.findProgramAddressSync(
            [Buffer.from(name), provider.wallet.publicKey.toBuffer(), Buffer.from(description)],
            program.programId
        );

        // Derive the token vault account address
        tokenVaultAccount = await getAssociatedTokenAddress(
            mint,
            savingsAccount,
            true
        );

        // Derive the user's associated token account (ATA)
        userAta = await getAssociatedTokenAddress(
            mint,
            provider.wallet.publicKey
        );
    })

    it("should withdraw SOL from the savings account", async () => {
        const name = "Test Savings";
        const description = "This is a test savings account";
        const isSol = false;
        const savingsType = { timeLockedSavings: {} };
        // const amount = new anchor.BN(100_000_000); // 100 USDC (assuming 6 decimals)
        const amount = new anchor.BN(0.1 * Math.pow(10, 6));
        const lockDuration = new anchor.BN(1); // 30 days in seconds

        // Create the user's associated token account (ATA) if it doesn't exist
        const userAta = await getAssociatedTokenAddress(
            mint, // Mint address (e.g., USDC or SOL)
            provider.wallet.publicKey // Owner of the ATA
        );

        const userAtaInfo = await provider.connection.getAccountInfo(userAta);
        if (!userAtaInfo) {
            console.log("Creating user ATA...");
            await createAssociatedTokenAccount(
                provider.connection,
                wallet.payer, // Payer
                mint, // Mint address
                provider.wallet.publicKey // Owner of the ATA
            );
        }

        const mintTx = await mintTo(provider.connection, wallet.payer, mint, userAta, provider.publicKey, 1_000_000_000);
        // console.log('Minted 10 tokens to contributor', mintTx);

        await program.methods.initializeSavings(
            name,
            description,
            isSol,
            savingsType,
            amount,
            lockDuration
        ).accountsPartial({
            signer: provider.wallet.publicKey,
            mint: mint,
            protocol: protocolAccount,
            tokenVaultAccount: tokenVaultAccount,
            savingsAccount: savingsAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        }).rpc();


        // Deposit SOL into the savings account
        await program.methods.depositSavings(
            name,
            description,
            savingsType,
            isSol,
            amount,
            lockDuration,
            null // unlockPrice (not used for SOL deposits)
        ).accountsPartial({
            signer: provider.wallet.publicKey,
            savingsAccount: savingsAccount,
            tokenVaultAccount: tokenVaultAccount,
            protocolState: protocolAccount,
            mint: mint,
            userAta: userAta,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        }).rpc()

        // const account = await program.account.savingsAccount.fetch(savingsAccount);
        // assert.ok(account.amount.eq(amount)); // Check if the amount matches
        // assert.ok(account.isSol === isSol);

        // // Wait for the lock duration to elapse
        await new Promise((resolve) => setTimeout(resolve, 6000));
        // // assert.ok(savingsAccounts.amount.eq(new anchor.BN(0))); // Check if the amount is 0 after withdrawal


        // Withdraw SOL from the savings account
        await program.methods.withdraw(
            name,
            description,
            savingsType,
            isSol,
            amount,
            null, // unlockPrice (not used for SOL deposits)
            lockDuration
        ).accountsPartial({
            signer: provider.wallet.publicKey,
            savingsAccount: savingsAccount,
            tokenVaultAccount: tokenVaultAccount,
            protocolState: protocolAccount,
            mint: mint,
            userAta: userAta,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
            .rpc();
        // Fetch the savings account and verify the withdrawal
        // const account = await program.account.savingsAccount.fetch(savingsAccount);
        // assert.ok(account.amount.eq(new anchor.BN(0))); // Check if the amount is 0 after withdrawal

    })
})