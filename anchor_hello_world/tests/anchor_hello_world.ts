import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorHelloWorld } from "../target/types/anchor_hello_world";

describe("anchor_hello_world", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorHelloWorld as Program<AnchorHelloWorld>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
