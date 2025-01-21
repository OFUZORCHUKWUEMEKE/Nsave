import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";

describe("withdraw-handler-tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.YourProgramName as Program<any>;

  let user: Keypair;
  let savingsAccount: Keypair;
  let mint: PublicKey;
  let userAta: PublicKey;
  let vaultAta: PublicKey;
  let protocolState: Keypair;

  const LOCK_DURATION = 5; // Lock duration in seconds for testing
  const TOKEN_AMOUNT = 1_000_000; // 1 token (assuming 6 decimals)

  before(async () => {
    user = Keypair.generate();
    savingsAccount = Keypair.generate();
    protocolState = Keypair.generate();

    // Airdrop SOL to user
    const connection = provider.connection;
    await connection.requestAirdrop(user.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(await connection.getLatestBlockhash());

    // Create mint and associated token accounts
    mint = await createMint(
      connection,
      user,
      user.publicKey,
      null,
      6 // 6 decimals
    );

    userAta = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint,
      user.publicKey
    );
    vaultAta = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint,
      savingsAccount.publicKey
    );

    // Mint tokens to the vault account
    await mintTo(connection, user, mint, vaultAta.address, user, TOKEN_AMOUNT);
  });

  it("transfers SOL after the lock period", async () => {
    const amount = 0.5 * LAMPORTS_PER_SOL; // 0.5 SOL

    // Call the withdraw_handler with is_sol = true
    const tx = await program.methods
      .withdrawHandler(new anchor.BN(amount), LOCK_DURATION)
      .accounts({
        savingsAccount: savingsAccount.publicKey,
        signer: user.publicKey,
        mint: mint,
        tokenVaultAccount: vaultAta.address,
        userAta: userAta.address,
        protocolState: protocolState.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Withdraw SOL Tx:", tx);

    // Check that the user received the SOL
    const userBalance = await provider.connection.getBalance(user.publicKey);
    assert(userBalance >= amount, "User did not receive SOL");
  });

  it("transfers tokens after the lock period", async () => {
    const amount = TOKEN_AMOUNT / 2; // Transfer 0.5 tokens

    // Call the withdraw_handler with is_sol = false
    const tx = await program.methods
      .withdrawHandler(new anchor.BN(amount), LOCK_DURATION)
      .accounts({
        savingsAccount: savingsAccount.publicKey,
        signer: user.publicKey,
        mint: mint,
        tokenVaultAccount: vaultAta.address,
        userAta: userAta.address,
        protocolState: protocolState.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Withdraw Token Tx:", tx);

    // Check that the user received the tokens
    const userAtaInfo = await getAccount(provider.connection, userAta.address);
    assert.equal(
      userAtaInfo.amount.toString(),
      amount.toString(),
      "User did not receive the correct token amount"
    );

    // Check that the vault balance decreased
    const vaultAtaInfo = await getAccount(provider.connection, vaultAta.address);
    assert.equal(
      vaultAtaInfo.amount.toString(),
      (TOKEN_AMOUNT - amount).toString(),
      "Vault balance mismatch"
    );
  });

  it("fails to withdraw before the lock period", async () => {
    const amount = 0.5 * LAMPORTS_PER_SOL;

    try {
      // Attempt to withdraw before the lock period has elapsed
      await program.methods
        .withdrawHandler(new anchor.BN(amount), LOCK_DURATION)
        .accounts({
          savingsAccount: savingsAccount.publicKey,
          signer: user.publicKey,
          mint: mint,
          tokenVaultAccount: vaultAta.address,
          userAta: userAta.address,
          protocolState: protocolState.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();

      assert.fail("Withdrawal should have failed before lock period");
    } catch (err) {
      console.log("Expected error:", err.message);
      assert(err.message.includes("FundsStillLocked"), "Unexpected error");
    }
  });
});
