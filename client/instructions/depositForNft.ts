import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface DepositForNftAccounts {
  bankAccount: PublicKey
  pdaAuth: PublicKey
  solVault: PublicKey
  bankAuth: PublicKey
  systemProgram: PublicKey
}

export function depositForNft(accounts: DepositForNftAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.bankAccount, isSigner: false, isWritable: true },
    { pubkey: accounts.pdaAuth, isSigner: false, isWritable: false },
    { pubkey: accounts.solVault, isSigner: false, isWritable: true },
    { pubkey: accounts.bankAuth, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
  ]
  const identifier = Buffer.from([21, 22, 224, 16, 180, 120, 218, 47])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
