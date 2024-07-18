import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';
import { TodoProg } from "../target/types/todo_prog";
import { expect } from "chai";
// import { utf8 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';

// Define constants for seeds

// const USER_TAG_STR = "USER_STATE";

// const TODO_TAG_STR = "TODO_STATE";
export const authorFilter = (authorBase58PublicKey) => ({
  memcmp: {
    offset: 8, // Discriminator.
    bytes: authorBase58PublicKey,
  },
});
describe("todo", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.TodoProg as Program<TodoProg>;

  it("Initialize User", async () => {
    await program.methods
      .initialize()
      .accounts({
        authority: payer.publicKey,
      })
      .rpc()
      .catch((e) => console.error(e));

    const res = await program.account.userProfile.all();

    expect(res.length).greaterThan(0);
  });
});
