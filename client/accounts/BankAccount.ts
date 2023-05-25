import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface BankAccountFields {
  bankAuth: PublicKey
  authBump: number
  solVaultBump: number | null
  nftAmount: types.NftAmountFields
}

export interface BankAccountJSON {
  bankAuth: string
  authBump: number
  solVaultBump: number | null
  nftAmount: types.NftAmountJSON
}

export class BankAccount {
  readonly bankAuth: PublicKey
  readonly authBump: number
  readonly solVaultBump: number | null
  readonly nftAmount: types.NftAmount

  static readonly discriminator = Buffer.from([
    43, 1, 157, 150, 152, 181, 247, 246,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("bankAuth"),
    borsh.u8("authBump"),
    borsh.option(borsh.u8(), "solVaultBump"),
    types.NftAmount.layout("nftAmount"),
  ])

  constructor(fields: BankAccountFields) {
    this.bankAuth = fields.bankAuth
    this.authBump = fields.authBump
    this.solVaultBump = fields.solVaultBump
    this.nftAmount = new types.NftAmount({ ...fields.nftAmount })
  }

  static async fetch(
    c: Connection,
    address: PublicKey
  ): Promise<BankAccount | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(PROGRAM_ID)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[]
  ): Promise<Array<BankAccount | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(PROGRAM_ID)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): BankAccount {
    if (!data.slice(0, 8).equals(BankAccount.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = BankAccount.layout.decode(data.slice(8))

    return new BankAccount({
      bankAuth: dec.bankAuth,
      authBump: dec.authBump,
      solVaultBump: dec.solVaultBump,
      nftAmount: types.NftAmount.fromDecoded(dec.nftAmount),
    })
  }

  toJSON(): BankAccountJSON {
    return {
      bankAuth: this.bankAuth.toString(),
      authBump: this.authBump,
      solVaultBump: this.solVaultBump,
      nftAmount: this.nftAmount.toJSON(),
    }
  }

  static fromJSON(obj: BankAccountJSON): BankAccount {
    return new BankAccount({
      bankAuth: new PublicKey(obj.bankAuth),
      authBump: obj.authBump,
      solVaultBump: obj.solVaultBump,
      nftAmount: types.NftAmount.fromJSON(obj.nftAmount),
    })
  }
}
