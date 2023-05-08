import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepositForNft } from "../target/types/deposit_for_nft";

describe("deposit-for-nft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DepositForNft as Program<DepositForNft>;

  const bankAuth = anchor.web3.Keypair.generate();
  const bankAccount = anchor.web3.Keypair.generate();

  let [pdaAuth, _pdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("pda-auth"),
      bankAccount.publicKey.toBuffer()
    ],
    program.programId
  );

  let [solVault, _solBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("sol-vault"),
      pdaAuth.toBuffer()
    ],
    program.programId
  );

  before(async () => {
    let res = await provider.connection.requestAirdrop(bankAuth.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });

  });

  it("Is Initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
      .accounts({
        bankAccount: bankAccount.publicKey,
        pdaAuth: pdaAuth,
        bankAuth: bankAuth.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([
        bankAccount,
        bankAuth,
      ]).rpc();
    console.log("Your transaction signature", tx);

    const banAccountInfo = await program.account.bankAccount.fetch(bankAccount.publicKey);
    console.log(`[Bank Account Info]\n`);
    console.log(banAccountInfo);
  });

  it("Deposit Sol For a NFT", async () => {
    const sol_amount = new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL);
    const deposit_tx = await program.methods.depositForNft(sol_amount)
      .accounts({
        bankAccount: bankAccount.publicKey,
        pdaAuth: pdaAuth,
        bankAuth: bankAuth.publicKey,
        solVault: solVault,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).signers([
        bankAuth,
      ]).rpc();

    const vault_balance = await provider.connection.getBalance(solVault);
    console.log(`[Vault Balance]\n${vault_balance}`);

    let bankAccountInfo = await program.account.bankAccount.fetch(bankAccount.publicKey);
    console.log(`[Bank Account Info]\n`);
    console.log(bankAccountInfo);
  })
});
