import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepositForNft } from "../target/types/deposit_for_nft";

describe("bank-for-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DepositForNft as Program<DepositForNft>;

  const bankAuth = anchor.web3.Keypair.generate();
  const bankAccount = anchor.web3.Keypair.generate();
  const clientAccount = anchor.web3.Keypair.generate();

  let [pdaAuth, _pdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("pda-auth"),
      bankAccount.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [solVault, _solBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("sol-vault"), pdaAuth.toBuffer()],
    program.programId
  );

  before(async () => {
    let res = await provider.connection.requestAirdrop(
      bankAuth.publicKey,
      100 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is Initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        bankAccount: bankAccount.publicKey,
        pdaAuth: pdaAuth,
        bankAuth: bankAuth.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([bankAccount, bankAuth])
      .rpc();
    console.log("Your transaction signature", tx);

    const banAccountInfo = await program.account.bankAccount.fetch(
      bankAccount.publicKey
    );
    console.log(`[Bank Account Info]\n`);
    console.log(banAccountInfo);
  });

  it("Deposit Sol For a NFT", async () => {
    const solAmount = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    const depositTx = await program.methods
      .depositForNft(solAmount)
      .accounts({
        bankAccount: bankAccount.publicKey,
        pdaAuth: pdaAuth,
        bankAuth: bankAuth.publicKey,
        solVault: solVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([bankAuth])
      .rpc();

    const vaultBalance = await provider.connection.getBalance(solVault);
    console.log(`[Vault Balance]\n${vaultBalance}`);

    let bankAccountInfo = await program.account.bankAccount.fetch(
      bankAccount.publicKey
    );
    console.log(`[Bank Account Info]\n`);
    console.log(bankAccountInfo);
  });

  it("Withdraw Sol for a NFT", async () => {
    const withdrawTx = await program.methods
      .withdrawHalfForNft()
      .accounts({
        bankAccount: bankAccount.publicKey,
        pdaAuth: pdaAuth,
        solVault: solVault,
        bankAuth: bankAuth.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        clientAccount: clientAccount.publicKey,
      })
      .signers([bankAuth])
      .rpc();

    const vaultBalance = await provider.connection.getBalance(solVault);
    console.log(`[Vault Balance]\n${vaultBalance}`);

    const bankAuthBalance = await provider.connection.getBalance(
      bankAuth.publicKey
    );
    console.log(`[Bank Auth Balance]\n${bankAuthBalance}`);

    const clientAccountBalance = await provider.connection.getBalance(
      clientAccount.publicKey
    );
    console.log(`[Client Account Balance]\n${clientAccountBalance}`);

    let bankAccountInfo = await program.account.bankAccount.fetch(
      bankAccount.publicKey
    );
    console.log(`[Bank Account Info]\n`);
    console.log(bankAccountInfo);
  });
});
