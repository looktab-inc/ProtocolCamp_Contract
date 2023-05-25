export type CustomError = InitializeError | NoNftLeftError

export class InitializeError extends Error {
  static readonly code = 6000
  readonly code = 6000
  readonly name = "InitializeError"
  readonly msg = "Failed to initialize contract."

  constructor(readonly logs?: string[]) {
    super("6000: Failed to initialize contract.")
  }
}

export class NoNftLeftError extends Error {
  static readonly code = 6001
  readonly code = 6001
  readonly name = "NoNftLeftError"
  readonly msg = "There is not NFTs left to withdraw sol"

  constructor(readonly logs?: string[]) {
    super("6001: There is not NFTs left to withdraw sol")
  }
}

export function fromCode(code: number, logs?: string[]): CustomError | null {
  switch (code) {
    case 6000:
      return new InitializeError(logs)
    case 6001:
      return new NoNftLeftError(logs)
  }

  return null
}
