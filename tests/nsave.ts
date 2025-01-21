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
} from '@solana/spl-token';
import { Nsave } from '../target/types/nsave';
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from '@coral-xyz/anchor/dist/cjs/native/system';
import { expect } from 'chai';


describe("Savings", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = anchor.workspace.Nsave as Program<Nsave>;

    const maker = anchor.web3.Keypair.generate();

    let data = {
        name: "emeke",
        description: "testing",
        isSol: false,
        savingsType: { timeLockedSavings: {} }
    };
    // const amount = new anchor.BN(2 * LAMPORTS_PER_SOL); // 1 SOL
    const amount = new anchor.BN(0.1 * Math.pow(10, 6));
    const lockDuration = new anchor.BN(0);
    const unlockPrice = null;

    let mint: anchor.web3.PublicKey;

    let userATA: anchor.web3.PublicKey;

    const wallet = provider.wallet as NodeWallet;

    const savings_account = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(data.name), maker.publicKey.toBuffer(), Buffer.from(data.description)], program.programId)[0];

    const protocol_account = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("protocol"), maker.publicKey.toBuffer()], program.programId)[0]

    const confirm = async (signature: string): Promise<string> => {
        const block = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            signature,
            ...block,
        });
        return signature;
    };

    it("Test Preparation", async () => {
        const airdrop = await provider.connection.requestAirdrop(maker.publicKey, 8 * anchor.web3.LAMPORTS_PER_SOL).then(confirm);
        // console.log('\nAirdropped 1 SOL to maker', airdrop);

        mint = await createMint(provider.connection, wallet.payer, provider.publicKey, provider.publicKey, 6);
        // console.log('Mint Created', mint.toBase58());

        userATA = (await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, mint, maker.publicKey)).address;

        const mintTx = await mintTo(provider.connection, wallet.payer, mint, userATA, provider.publicKey, 1_000_000_0);
        // console.log('Minted 10 tokens to contributor', mintTx);
        console.log("SavingsAccount before Initialization", (await provider.connection.getBalance(savings_account)));
        console.log("SignerAccount before Initialization", (await provider.connection.getBalance(maker.publicKey)));
    });

    it("Initialize Savings", async () => {
        const vault = getAssociatedTokenAddressSync(mint, savings_account, true);

        const tx = await program.methods.initializeSavings(
            data.name,
            data.description,
            data.isSol,
            data.savingsType,
            amount,
            lockDuration,
            unlockPrice
        ).accountsPartial({
            signer: maker.publicKey,
            mint,
            protocol: protocol_account,
            tokenVaultAccount: vault,
            savingsAccount: savings_account,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID
        }).signers([maker]).rpc().then(confirm);
        const savings = await provider.connection.getBalance(savings_account);
        // console.log(savings)

        // console.log("SavingsAccount after initialization", (await provider.connection.getBalance(savings_account)));
        // console.log("Signer sent this", (await provider.connection.getBalance(maker.publicKey)));
        console.log("User ATA", await provider.connection.getTokenAccountBalance(userATA));
        // expect()

    })

    it("Deposit SOL", async () => {
        const vault = getAssociatedTokenAddressSync(mint, savings_account, true);

        const tx = await program.methods.depositSavings(
            data.name,
            data.description,
            data.savingsType,
            data.isSol,
            amount,
            lockDuration,
            unlockPrice
        ).accountsPartial({
            signer: maker.publicKey,
            savingsAccount: savings_account,
            tokenVaultAccount: vault,
            protocolState: protocol_account,
            userAta: userATA,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID, mint
        }).signers([maker]).rpc().then(confirm);

        // console.log('Vault Balance', (await provider.connection.getTokenAccountBalance(userATA)).value.amount);
        // console.log("SavingsAccount", (await provider.connection.getBalance(savings_account)));
        // console.log("Signer sent this", (await provider.connection.getBalance(maker.publicKey)));

        console.log("User ATA", await provider.connection.getTokenAccountBalance(userATA));

    })

    it("withdraw", async () => {
        const vault = getAssociatedTokenAddressSync(mint, savings_account, true);
        // console.log('Vault Balance', (await provider.connection.getTokenAccountBalance(vault)).value.amount);

        const tx = program.methods.withdraw(
            data.name,
            data.description,
            data.savingsType,
            data.isSol,
            amount,
            unlockPrice,
            lockDuration,

        ).accountsPartial({
            signer: maker.publicKey,
            savingsAccount: savings_account,
            mint,
            tokenVaultAccount: vault,
            protocolState: protocol_account,
            userAta: userATA,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID
        }).rpc().then(confirm);
        console.log('Vault Balance', (await provider.connection.getTokenAccountBalance(vault)).value.amount);
        console.log('USER ATA', (await provider.connection.getTokenAccountBalance(userATA)).value.amount);
        // console.log("SavingsAccount", (await provider.connection.getBalance(savings_account)));
        // console.log("Signer sent this", (await provider.connection.getBalance(maker.publicKey)));
    })
})