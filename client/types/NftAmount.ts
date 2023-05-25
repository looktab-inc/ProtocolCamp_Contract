import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@project-serum/borsh"

export interface NftAmountFields {
  total: number
  remained: number
}

export interface NftAmountJSON {
  total: number
  remained: number
}

export class NftAmount {
  readonly total: number
  readonly remained: number

  constructor(fields: NftAmountFields) {
    this.total = fields.total
    this.remained = fields.remained
  }

  static layout(property?: string) {
    return borsh.struct([borsh.u16("total"), borsh.u16("remained")], property)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new NftAmount({
      total: obj.total,
      remained: obj.remained,
    })
  }

  static toEncodable(fields: NftAmountFields) {
    return {
      total: fields.total,
      remained: fields.remained,
    }
  }

  toJSON(): NftAmountJSON {
    return {
      total: this.total,
      remained: this.remained,
    }
  }

  static fromJSON(obj: NftAmountJSON): NftAmount {
    return new NftAmount({
      total: obj.total,
      remained: obj.remained,
    })
  }

  toEncodable() {
    return NftAmount.toEncodable(this)
  }
}
