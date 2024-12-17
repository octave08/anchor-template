import { Program, setProvider } from "@coral-xyz/anchor"
import type { HelloAnchor } from "@target/types/hello_anchor"
import { BankrunProvider } from "anchor-bankrun"
import { startAnchor } from "solana-bankrun"

import IDL from "@target/idl/hello_anchor.json"

describe("hello-anchor", () => {
  let program: Program<HelloAnchor>

  beforeAll(async () => {
    const context = await startAnchor("", [], [])
    const provider = new BankrunProvider(context)
    setProvider(provider)

    program = new Program<HelloAnchor>(IDL as HelloAnchor, provider)
  })

  test("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc()

    console.log("Your transaction signature", tx)
  })
})
