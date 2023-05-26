import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface WithdrawForExpiredAccounts {
  bankAccount: PublicKey
  pdaAuth: PublicKey
  solVault: PublicKey
  bankAuth: PublicKey
  systemProgram: PublicKey
  clientAccount: PublicKey
}

export function withdrawForExpired(accounts: WithdrawForExpiredAccounts) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.bankAccount, isSigner: false, isWritable: false },
    { pubkey: accounts.pdaAuth, isSigner: false, isWritable: false },
    { pubkey: accounts.solVault, isSigner: false, isWritable: true },
    { pubkey: accounts.bankAuth, isSigner: true, isWritable: true },
    { pubkey: accounts.systemProgram, isSigner: false, isWritable: false },
    { pubkey: accounts.clientAccount, isSigner: false, isWritable: true },
  ]
  const identifier = Buffer.from([90, 123, 129, 146, 41, 253, 69, 183])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId: PROGRAM_ID, data })
  return ix
}
