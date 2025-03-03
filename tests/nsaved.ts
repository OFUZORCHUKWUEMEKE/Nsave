// import * as anchor from '@coral-xyz/anchor';
// import type { Program } from '@coral-xyz/anchor';
// import type NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';
// import {
//     ASSOCIATED_TOKEN_PROGRAM_ID,
//     TOKEN_PROGRAM_ID,
//     createMint,
//     getAssociatedTokenAddressSync,
//     getOrCreateAssociatedTokenAccount,
//     mintTo,
//     getAssociatedTokenAddress
// } from '@solana/spl-token';
// import { Nsave } from '../target/types/nsave';
// import { PublicKey, SystemProgram } from '@solana/web3.js';
// import { LAMPORTS_PER_SOL } from "@solana/web3.js";
// import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';
// import { assert, expect } from 'chai';

// describe('savings-platform', () => {
//     const provider = anchor.AnchorProvider.env();

//     anchor.setProvider(provider);

//     const program = anchor.workspace.Nsave as Program<Nsave>;

//     let mint: PublicKey;
//     let tokenVaultAccount: PublicKey;
//     let savingsAccount: PublicKey;
//     let protocolAccount: PublicKey;


//     const wallet = provider.wallet as NodeWallet;

//     before(async () => {

//         mint = await createMint(
//             provider.connection,
//             wallet.payer,
//             provider.wallet.publicKey,
//             null,
//             6 // Decimals for USDC
//         );


//         // Derive the protocol account address
//         [protocolAccount] = await PublicKey.findProgramAddressSync(
//             [Buffer.from("protocol"), provider.wallet.publicKey.toBuffer()],
//             program.programId
//         );

//         const name = "Test Savings";
//         const description = "This is a test savings account";
//         [savingsAccount] = await PublicKey.findProgramAddressSync(
//             [Buffer.from(name), provider.wallet.publicKey.toBuffer(), Buffer.from(description)],
//             program.programId
//         );

//         tokenVaultAccount = await getAssociatedTokenAddress(
//             mint,
//             savingsAccount,
//             true
//         );
//     })

//     it("should initialize a savigs account", async () => {
//         const name = "Test Savings";
//         const description = "This is a test savings account";
//         const isSol = false;
//         const savingsType = { timeLockedSavings: {} };
//         const amount = new anchor.BN(1000); // 1000 USDC
//         const lockDuration = new anchor.BN(30 * 24 * 60 * 60); // 30 days in seconds


//         await program.methods.initializeSavings(
//             name,
//             description,
//             isSol,
//             savingsType,
//             amount,
//             lockDuration
//         ).accountsPartial({
//             signer: provider.wallet.publicKey,
//             mint: mint,
//             protocol: protocolAccount,
//             tokenVaultAccount: tokenVaultAccount,
//             savingsAccount: savingsAccount,
//             tokenProgram: TOKEN_PROGRAM_ID,
//             systemProgram: SystemProgram.programId,
//             associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//         }).rpc();


//         // Fetch the savings account and verify its data
//         const account = await program.account.savingsAccount.fetch(savingsAccount);
//         assert.ok(account.name === name);
//         assert.ok(account.description === description);
//         assert.ok(account.owner.equals(provider.wallet.publicKey));
//         // assert.ok(account.amount === amount.toNumber());
//         // assert.ok(account.lockDuration === lockDuration.toNumber());
//     })


// })