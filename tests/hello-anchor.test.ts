import { BN, Program, type Wallet, setProvider } from "@coral-xyz/anchor"
import type { HelloAnchor } from "@target/types/hello_anchor"
import { BankrunProvider } from "anchor-bankrun"
import { startAnchor } from "solana-bankrun"

import { Keypair } from "@solana/web3.js"
import IDL from "@target/idl/hello_anchor.json"

describe("hello-anchor", () => {
  let program: Program<HelloAnchor>
  let wallet: Wallet

  beforeAll(async () => {
    const context = await startAnchor("", [], [])
    const provider = new BankrunProvider(context)
    setProvider(provider)

    wallet = provider.wallet
    program = new Program<HelloAnchor>(IDL as HelloAnchor, provider)
  })

  test("initialize", async () => {
    const newKeypair = new Keypair()
    const data = new BN(42)

    const tx = await program.methods
      .initialize(data)
      .accounts({
        newAccount: newKeypair.publicKey,
        signer: wallet.publicKey,
      })
      .signers([newKeypair])
      .rpc()

    const newAccount = await program.account.memo.fetch(newKeypair.publicKey)

    console.log("Your transaction signature:", tx)
    console.log("On-chain data is:", newAccount.data.toString())

    expect(newAccount.data.eq(data)).toBeTruthy()
  })
})
